/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::db::db_model::{CalcParam, CalcRule};
use anyhow::{Context, Result};
use chrono::Utc;
use common_lib::config::InfluxConfig;
use common_lib::influxdb_utils::{InfluxDBManager, LocValue};
use common_lib::models::{AggregationConfig, InfluxQueryConfig};
use common_lib::mongo_utils::MongoDBManager;
use common_lib::redis_pool_utils::RedisOp;
use common_lib::servlet_common::{CalcCache, CalcParamCache};
use common_lib::time_utils::get_next_time;
use common_lib::ut::{calc_bucket_name, calc_collection_name, calc_measurement};
use cron::Schedule;
use futures_lite::StreamExt;
use influxdb2_structmap::value::Value;
use log::{error, info};
use mongodb::{bson::doc, options::FindOptions};
use quick_js::JsValue;
use r2d2_redis::redis::Commands;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::Json;
use serde_json::json;
use serde_json::Map;
use sqlx::MySqlPool;
use std::collections::{BTreeMap, HashMap};

pub struct CalcRunBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
    pub mongo: &'static MongoDBManager,
    pub config: InfluxConfig,
}

impl CalcRunBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool, mongo: &'static MongoDBManager, config: InfluxConfig) -> Self {
        CalcRunBiz { redis, mysql, mongo, config }
    }

    pub async fn QueryRuleExData(&self,
                                 rule_id: i64,
                                 start_time: i64,
                                 end_time: i64,
                                 mongo_config: common_lib::config::MongoConfig,
    ) -> rocket::response::status::Custom<Json<serde_json::Value>> {
        // 构建集合名称
        let collection_name = calc_collection_name(&mongo_config.collection.unwrap(), rule_id);

        // 构建查询条件
        let filter = doc! {
            "calc_rule_id": rule_id as i64,
            "ex_time": {
                "$gte": start_time,
                "$lte": end_time
            }
        };


        // 执行查询
        match self.mongo.collection(&collection_name).find(filter).await {
            Ok(cursor) => {
                let mut results = Vec::new();
                let mut cursor = cursor;

                while let Ok(Some(doc)) = cursor.try_next().await {
                    results.push(doc);
                }

                let success_json = json!({
                    "code": 20000,
                    "message": "查询成功",
                    "data": results
                });
                Custom(Status::Ok, Json(success_json))
            }
            Err(e) => {
                error!("查询失败: {:?}", e);
                let error_json = json!({
                    "code": 40000,
                    "message": "查询失败"
                });
                Custom(Status::Ok, Json(error_json))
            }
        }
    }

    pub async fn start(&self, role_id: i64) -> Result<bool> {
        let calc_cache = self.refresh_rule(role_id).await?;


        let option = get_next_time(&calc_cache.cron).unwrap();
        self.redis.add_zset("calc_queue", calc_cache.id.to_string().as_str(), option as f64)
            .map_err(|e| anyhow::anyhow!("Failed to add to redis zset: {}", e))?;

        // 4. 更新数据库中的状态
        let sql = "UPDATE calc_rules SET start = true WHERE id = ?";
        sqlx::query(sql)
            .bind(role_id.to_string())
            .execute(&self.mysql)
            .await
            .with_context(|| format!("Failed to update calc rule status for id: {}", role_id))?;

        Ok(true)
    }

    pub async fn stop(&self, role_id: i64) -> Result<bool> {
        let sql = "select * from calc_rules where id = ?";

        let record = sqlx::query_as::<_, CalcRule>(sql)
            .bind(role_id.to_string())
            .fetch_optional(&self.mysql)
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch calc rule from table '{}' with id {:?}",
                    "calc_rules",
                    role_id
                )
            })?;

        if let Some(calc_rule) = record {
            let json_data = json::json!({
                "id": calc_rule.id
            });
            let json_str = json::to_string(&json_data).unwrap();

            self.redis.delete_zset("calc_queue", &json_str).unwrap();
            self.redis.delete_hash_field("calc_cache", &role_id.to_string()).unwrap();
            self.redis
                .delete(&format!("calc_queue_param:{}", role_id))
                .unwrap();

            let sql_update = "UPDATE calc_rules SET start = ? WHERE id = ?";
            sqlx::query(sql_update)
                .bind(false)
                .bind(role_id.to_string())
                .execute(&self.mysql)
                .await
                .with_context(|| {
                    format!(
                        "Failed to update calc rule in table '{}' with id {:?}",
                        "calc_rules",
                        role_id
                    )
                })?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn InitMongoCollection(&self, d: &CalcRule, collection: String) {
        let string = calc_collection_name(collection.as_str(), d.id.unwrap());
        self.mongo.create_collection(string.as_str()).await.unwrap();
    }

    pub async fn refresh_rule(&self, role_id: i64) -> Result<CalcCache> {
        let sql = "select * from calc_rules where id = ?";

        let record = sqlx::query_as::<_, CalcRule>(sql).bind(role_id.to_string()).fetch_optional(&self.mysql).await.with_context(|| {
            format!(
                "Failed to fetch calc rule from table '{}' with id {:?}",
                "calc_rules",
                role_id
            )
        })?;

        let calc_rule = record.ok_or_else(|| anyhow::anyhow!("Calc rule not found for id: {}", role_id))?;

        let sql2 = "select * from calc_params where calc_rule_id = ?";
        let record2 = sqlx::query_as::<_, CalcParam>(sql2).bind(role_id.to_string()).fetch_all(&self.mysql).await.with_context(|| {
            format!(
                "Failed to fetch calc params from table '{}' for calc_rule_id {:?}",
                "calc_params",
                role_id
            )
        })?;

        let calc_param_cache: Vec<CalcParamCache> = record2.into_iter().filter_map(|param| {
            Some(CalcParamCache {
                protocol: param.protocol?,
                identification_code: param.identification_code?,
                device_uid: param.device_uid?,
                name: param.name?,
                signal_name: param.signal_name?,
                reduce: param.reduce?,
                calc_rule_id: param.calc_rule_id?,
                signal_id: param.signal_id?,
            })
        }).collect();

        let calc_cache = CalcCache {
            id: calc_rule.id.unwrap(),
            param: Some(calc_param_cache),
            cron: calc_rule.cron.unwrap(),
            script: calc_rule.script.unwrap(),
            offset: calc_rule.offset.unwrap(),
        };

        json::to_string(&calc_cache).map_err(|e| anyhow::anyhow!("Failed to serialize calc cache: {}", e)).and_then(|s| {
            self.redis.set_hash("calc_cache", calc_cache.id.to_string().as_str(), s.as_str()).map_err(|e| anyhow::anyhow!("Failed to set redis hash: {}", e))?;
            Ok(calc_cache)
        })
    }

    pub async fn mock_calc(&self, start_time: i64, end_time: i64, id: i64) -> Option<String> {
        // Get calc cache from Redis
        let result = match self.redis.get_hash("calc_cache", &id.to_string()) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get from redis: {:?}", e);
                return None;
            }
        };

        // Deserialize calc cache
        let calc_cache: CalcCache = match serde_json::from_str(&result.unwrap()) {
            Ok(cache) => cache,
            Err(e) => {
                error!("Failed to deserialize calc cache: {:?}", e);
                return None;
            }
        };

        let influxdb = InfluxDBManager::new(
            &self.config.clone().host.unwrap(),
            self.config.clone().port?,
            &self.config.clone().org.unwrap(),
            &self.config.clone().token.unwrap(),
        );
        let mut m: HashMap<String, LocValue> = HashMap::new();

        for cache in calc_cache.param.unwrap().iter() {
            if cache.reduce == "原始" {
                let field = vec![cache.signal_id.to_string()];
                let config = InfluxQueryConfig {
                    bucket: calc_bucket_name(self.config.clone().bucket.unwrap().clone().as_ref(), &cache.protocol, cache.device_uid),
                    measurement: calc_measurement(cache.device_uid.to_string().as_str(), &cache.identification_code, &cache.protocol),
                    fields: field,
                    aggregation: AggregationConfig {
                        every: 1,
                        function: "mean".to_string(),
                        create_empty: false,
                    },
                    start_time,
                    end_time,
                    reduce: cache.reduce.clone(),
                    device_uid: None,
                    protocol: None,
                };

                let query_string = config.generate_flux_query();
                log::info!("influxdb query line = {}", query_string);


                let vec1 = influxdb
                    .query_with_string(query_string)
                    .await
                    .unwrap();

                let mut v: HashMap<i64, f64> = HashMap::new();
                if vec1.is_empty() {
                    info!("no data");
                } else {
                    for record in vec1 {
                        // 打印每条记录的详细信息

                        let va = record.values.get("_value").unwrap();
                        let mut vaa: f64 = 0.0;
                        match va {
                            Value::Unknown => {}
                            Value::String(_) => {}
                            Value::Double(fa) => {
                                vaa = fa.0;
                            }
                            Value::Bool(_) => {}
                            Value::Long(_) => {}
                            Value::UnsignedLong(_) => {}
                            Value::Duration(_) => {}
                            Value::Base64Binary(_) => {}
                            Value::TimeRFC(_) => {}
                        }

                        let time = record.values.get("_time").unwrap();
                        let mut t: i64 = 0;
                        match time {
                            influxdb2_structmap::value::Value::Unknown => {}
                            Value::String(_) => {}
                            Value::Double(_) => {}
                            Value::Bool(_) => {}
                            Value::Long(_) => {}
                            Value::UnsignedLong(_) => {}
                            Value::Duration(_) => {}
                            Value::Base64Binary(_) => {}
                            Value::TimeRFC(tt) => {
                                let beijing_time = tt.with_timezone(
                                    &chrono::FixedOffset::east(
                                        8 * 3600,
                                    ),
                                );
                                t = beijing_time.timestamp();
                            }
                        }
                        v.insert(t, vaa);
                    }
                    m.insert(cache.name.clone(), LocValue::Map(v));
                }
            } else {
                let field = vec![cache.signal_id.to_string()];
                let config = InfluxQueryConfig {
                    bucket: calc_bucket_name(self.config.clone().bucket.unwrap().clone().as_ref(), &cache.protocol, cache.device_uid),
                    measurement: calc_measurement(cache.device_uid.to_string().as_str(), &cache.identification_code, &cache.protocol),
                    fields: field,
                    start_time,
                    end_time,
                    reduce: cache.reduce.clone(),
                    device_uid: None,
                    aggregation: AggregationConfig {
                        every: 1,
                        function: "mean".to_string(),
                        create_empty: false,
                    },
                    protocol: None,
                };

                let query_string = config.generate_flux_reduce();
                log::info!("influxdb query line = {}", query_string);

                let vec1 = influxdb
                    .query_with_string(query_string)
                    .await
                    .unwrap();

                if vec1.is_empty() {
                    info!("no data");
                } else {
                    for record in vec1 {
                        let va = record.values.get("_value").unwrap();
                        let mut vaa: f64 = 0.0;
                        match va {
                            Value::Unknown => {}
                            Value::String(_) => {}
                            Value::Double(fa) => {
                                vaa = fa.0;
                            }
                            Value::Bool(_) => {}
                            Value::Long(_) => {}
                            Value::UnsignedLong(_) => {}
                            Value::Duration(_) => {}
                            Value::Base64Binary(_) => {}
                            Value::TimeRFC(_) => {}
                        }

                        m.insert(
                            cache.name.clone(),
                            LocValue::Scalar(vaa),
                        );
                    }
                }
            };
        }

        let pa = serde_json::to_string(&m).unwrap();


        let fff = {
            let context = quick_js::Context::new().unwrap();
            context.eval(calc_cache.script.as_str()).unwrap();

            let js_code_2 = r#"
    function main2(data) {
        return JSON.parse(data);
    }"#;
            context.eval(js_code_2).unwrap();
            let js_code_3 = r#"
    function main3(data) {
        return JSON.stringify(data);
    }"#;
            context.eval(js_code_3).unwrap();
            let value2 = context.call_function("main2", [pa.clone()]).unwrap();
            let value = context.call_function("main", [value2]).unwrap();
            context.call_function("main3", [value]).unwrap()
        };
        return match fff {
            JsValue::String(json_str) => {


                // Update mock value in database
                if let Ok(mut old) = sqlx::query_as::<_, CalcRule>("SELECT * FROM calc_rules WHERE id = ?")
                    .bind(calc_cache.id)
                    .fetch_one(&self.mysql)
                    .await
                {
                    old.mock_value = Some(json_str.clone());

                    let _ = sqlx::query("UPDATE calc_rules SET mock_value = ? WHERE id = ?")
                        .bind(&old.mock_value)
                        .bind(old.id)
                        .execute(&self.mysql)
                        .await;
                }

                Some(json_str)
            }

            _ => {
                None
            }
        };
        None
    }
}
