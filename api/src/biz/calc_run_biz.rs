use crate::db::db_model::{CalcParam, CalcRule};
use anyhow::{Context, Result};
use common_lib::mongo_utils::MongoDBManager;
use common_lib::redis_pool_utils::RedisOp;
use common_lib::servlet_common::{CalcCache, CalcParamCache};
use common_lib::ut::calc_collection_name;
use rocket::serde::json;
use sqlx::MySqlPool;

pub struct CalcRunBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
    pub mongo: MongoDBManager,

}

impl CalcRunBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool, mongo_dbmanager: MongoDBManager) -> Self {
        CalcRunBiz { redis, mysql, mongo: mongo_dbmanager }
    }

    pub async fn start(&self, role_id: u64) {}
    pub async fn stop(&self, role_id: u64) {}

    pub async fn InitMongoCollection(&self, d: &CalcRule, collection: String) {
        let string = calc_collection_name(collection.as_str(), d.id.unwrap() as i32);
        self.mongo.create_collection(string.as_str()).await.unwrap();
    }

    pub async fn refresh_rule(&self, role_id: u64) -> Result<CalcCache> {
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
            param: calc_param_cache,
            cron: calc_rule.cron.unwrap(),
            script: calc_rule.script.unwrap(),
            offset: calc_rule.offset.unwrap(),
        };

        json::to_string(&calc_cache).map_err(|e| anyhow::anyhow!("Failed to serialize calc cache: {}", e)).and_then(|s| {
            self.redis.set_hash("calc_cache", calc_cache.id.to_string().as_str(), s.as_str()).map_err(|e| anyhow::anyhow!("Failed to set redis hash: {}", e))?;
            Ok(calc_cache)
        })
    }
}
