use crate::storage_handler::{calc_bucket_name, calc_measurement};
use bson::{Bson, Document};
use chrono::Utc;
use common_lib::influxdb_utils::InfluxDBManager;
use common_lib::models::{AggregationConfig, CalcCache, InfluxQueryConfig};
use common_lib::mongo_utils::MongoDBManager;
use common_lib::redis_handler::RedisWrapper;
use cron::Schedule;
use futures_util::StreamExt;
use influxdb2_structmap::value::Value;
use log::{debug, error, info, trace};
use quick_js::{Context, JsValue};
use serde::de::{self, MapAccess, Visitor};
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_str, Error};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::waring_dealy_handler::handler_waring_delay_string;
use crate::waring_handler::calc_collection_name;
use common_lib::config::InfluxConfig;
use common_lib::redis_pool_utils::RedisOp;
use common_lib::time_utils::local_to_utc;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::{Channel, Connection};

#[derive(Debug)]
enum LocValue {
    Map(HashMap<i64, f64>),
    Scalar(f64),
}

impl Serialize for LocValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LocValue::Map(map) => {
                let mut ser_map = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    ser_map.serialize_entry(k, v)?;
                }
                ser_map.end()
            }
            LocValue::Scalar(scalar) => serializer.serialize_f64(*scalar),
        }
    }
}

impl<'de> Deserialize<'de> for LocValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LocValueVisitor;

        impl<'de> Visitor<'de> for LocValueVisitor {
            type Value = LocValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map or a scalar value")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LocValue::Scalar(value))
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some((key, value)) = access.next_entry()? {
                    map.insert(key, value);
                }
                Ok(LocValue::Map(map))
            }
        }

        deserializer.deserialize_any(LocValueVisitor)
    }
}
pub async fn calc_handler(
    msg: String,
    redis_wrapper: &RedisOp,
    bucket_name: String,

    host: &str,
    port: u16,
    org: &str,
    token: &str,
    mongo: &MongoDBManager,
    calc_collection: String,
) {
    let my_map: Result<HashMap<String, i64>, serde_json::Error> = from_str(msg.as_str());

    match my_map {
        Ok(ref map) => {
            if let Some(id_value) = my_map.expect("REASON").get("id") {
                let id_str = id_value.to_string(); // 将 i64 转换为 String
                info!("id as string: {}", id_str);
                let influxdb = InfluxDBManager::new(host, port, org, token);

                let option = redis_wrapper
                    .get_hash("calc_cache", id_str.as_str())
                    .unwrap();
                if let None = option {
                } else if let Some(result) = option {
                    let ccc_res: Result<CalcCache, serde_json::Error> = from_str(result.as_str());
                    if let Ok(ccc) = ccc_res {
                        let next_time = get_next_time(ccc.cron.as_str()).unwrap();

                        info!("next time: {}", next_time);

                        let k = format!("calc_queue_param:{}", id_value);

                        info!("k: {}", k);
                        let value = redis_wrapper.get_string(k.as_str()).unwrap();
                        if value.is_some() {
                            match value.unwrap().parse::<i64>() {
                                Ok(pre_time) => {
                                    let mut m: HashMap<String, LocValue> = HashMap::new();
                                    for cache in ccc.param.unwrap() {
                                        let bkn = calc_bucket_name(
                                            bucket_name.as_str(),
                                            cache.protocol.as_str(),
                                            cache.device_uid,
                                        );
                                        let mm = calc_measurement(
                                            cache.device_uid.to_string().as_str(),
                                            cache.identification_code.as_str(),
                                            cache.protocol.as_str(),
                                        );
                                        if "原始" == cache.reduce.as_str() {
                                            let influxdb_config = InfluxQueryConfig {
                                                bucket: bkn,
                                                measurement: mm,
                                                fields: vec![cache.signal_id.to_string()],
                                                start_time: pre_time - ccc.offset,
                                                end_time: pre_time,
                                                aggregation: AggregationConfig {
                                                    every: 1,
                                                    function: "mean".to_string(),
                                                    create_empty: false,
                                                },
                                                reduce: cache.reduce,
                                            };
                                            let query_string =
                                                influxdb_config.generate_flux_query();

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
                                                        Value::Unknown => {}
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
                                                m.insert(cache.name, LocValue::Map(v));
                                            }
                                        } else {
                                            let influxdb_config = InfluxQueryConfig {
                                                bucket: bkn,
                                                measurement: mm,
                                                fields: vec![cache.signal_id.to_string()],
                                                start_time: pre_time - ccc.offset,
                                                end_time: pre_time,
                                                aggregation: AggregationConfig {
                                                    every: 1,
                                                    function: "mean".to_string(),
                                                    create_empty: false,
                                                },
                                                reduce: cache.reduce,
                                            };
                                            let query_string =
                                                influxdb_config.generate_flux_reduce();
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
                                        }
                                    }

                                    let pa = serde_json::to_string(&m).unwrap();
                                    trace!("m = {}", pa);

                                    let context = Context::new().unwrap();

                                    context.eval(ccc.script.as_str()).unwrap();

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
                                    let value2 =
                                        context.call_function("main2", [pa.clone()]).unwrap();

                                    let value = context.call_function("main", [value2]).unwrap();

                                    let fff = context.call_function("main3", [value]).unwrap();

                                    match fff {
                                        JsValue::String(json_str) => {
                                            let document1 =
                                                json_str_to_document(&json_str).unwrap();
                                            trace!("document1 = {}", document1);

                                            // let document2: Document = bson::to_document(&json_value).expect("Failed to convert to Document");
                                            let document3: Document =
                                                json_str_to_document(pa.clone().as_str()).unwrap();

                                            let mut document = Document::new();
                                            document.insert("calc_rule_id", id_str);
                                            document.insert("ex_time", next_time);
                                            document.insert("start_time", next_time - ccc.offset);
                                            document.insert("end_time", next_time);
                                            document.insert("param", document3);
                                            document.insert("script", Bson::String(ccc.script));
                                            document.insert("result", document1);

                                            trace!("value2 = {:?}", document);

                                            let string = calc_collection_name(
                                                calc_collection.as_str(),
                                                ccc.id as i32,
                                            );
                                            let collection =
                                                mongo.db.collection::<Document>(string.as_str());

                                            collection
                                                .insert_one(&document.clone(), None)
                                                .await
                                                .unwrap();

                                            // msg

                                            redis_wrapper
                                                .set_string(
                                                    k.as_str(),
                                                    next_time.to_string().as_str(),
                                                )
                                                .unwrap();
                                            redis_wrapper
                                                .add_zset(
                                                    "calc_queue",
                                                    msg.as_str(),
                                                    next_time as f64,
                                                )
                                                .unwrap();
                                        }
                                        _ => {
                                            error!("存储异常");
                                        }
                                    }
                                }
                                Err(e) => println!("Failed to parse the string: {}", e),
                            }
                        }
                    } else if let Err(e) = ccc_res {
                        error!("{}", e);
                    }
                }
            } else {
                error!("Key 'id' not found");
            }
        }
        Err(err) => {
            error!("Error decoding JSON: {}", err);
        }
    }
}

fn json_str_to_document(json_str: &str) -> Result<Document, Box<dyn std::error::Error>> {
    // 解析 JSON 字符串为 `serde_json::Value`
    let json_value: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to parse JSON string: {}", e);
            return Err(Box::new(e));
        }
    };

    // 如果是数组类型，取第一个元素处理
    let json_value = match json_value {
        serde_json::Value::Array(arr) => {
            if let Some(first) = arr.first() {
                first.clone()
            } else {
                let err_msg = "Array is empty";
                error!("{}", err_msg);
                return Err(err_msg.into());
            }
        }
        _ => json_value, // 如果是对象或其他类型，直接返回
    };

    // 尝试将 `serde_json::Value` 转换为 BSON
    let bson = match Bson::try_from(json_value) {
        Ok(bson) => bson,
        Err(e) => {
            error!("Failed to convert JSON value to BSON: {}", e);
            return Err(Box::new(e));
        }
    };

    // 如果 BSON 是 Document 类型，则返回它
    if let Bson::Document(doc) = bson {
        Ok(doc)
    } else {
        let err_msg = "Parsed JSON is not a BSON document";
        error!("{}", err_msg);
        Err(err_msg.into())
    }
}

fn get_next_time(cron_expr: &str) -> Option<i64> {
    // 解析 cron 表达式
    let schedule = Schedule::from_str(cron_expr).ok()?;

    // 计算下一次执行时间
    let time = Utc::now();

    let beijing_time = time.with_timezone(&chrono::FixedOffset::east(8 * 3600));

    println!("当前时间: {}", beijing_time);
    if let Some(next_time) = schedule.after(&time).next() {
        // 返回时间戳（秒）
        Some(next_time.timestamp())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDateTime, TimeZone, Utc};
    use common_lib::config::{get_config, read_config, read_config_tb};
    use common_lib::init_logger;
    use common_lib::mongo_utils::{get_mongo, init_mongo};
    use common_lib::rabbit_utils::init_rabbitmq_with_config;
    use common_lib::redis_handler::{get_redis_instance, init_redis};
    use common_lib::redis_pool_utils::create_redis_pool_from_config;

    #[tokio::test]
    async fn cc() {
        let s = r#"{"id":1}"#;

        init_logger();

        let result = read_config("app-local.yml").await.unwrap();
        let config = get_config().await.unwrap();

        let redis_config = config.redis_config.clone();
        let influxdb = config.influx_config.clone().unwrap();
        init_redis(redis_config).await.unwrap();
        init_rabbitmq_with_config(config.mq_config.clone())
            .await
            .unwrap();
        let mongo_config = config.mongo_config.clone().unwrap();

        init_mongo(mongo_config.clone()).await.unwrap();

        let mongo_manager_wrapper = get_mongo().await.unwrap();
        let config1 = read_config_tb("app-local.yml");
        let pool = create_redis_pool_from_config(&config1.redis_config);

        let redisOp = RedisOp { pool };
        calc_handler(
            s.to_string(),
            &redisOp,
            influxdb.bucket.unwrap(),
            influxdb.host.unwrap().as_str(),
            influxdb.port.unwrap(),
            influxdb.org.unwrap().as_str(),
            influxdb.token.unwrap().as_str(),
            &mongo_manager_wrapper.clone(),
            mongo_config.collection.clone().unwrap(),
        )
        .await;
    }

    #[test]
    fn test_get_next_time() {
        // 定义一个 cron 表达式 (例如每分钟执行一次)
        let cron_expr = "0 0/2 * * * ?";

        // 获取当前时间
        let now = common_lib::time_utils::local_to_utc();

        // 获取下一次执行时间
        if let Some(next_time) = get_next_time(cron_expr) {
            let next_time_naive = NaiveDateTime::from_timestamp(next_time, 0);
            let utc_time = Utc.from_utc_datetime(&next_time_naive);

            // 转换为北京时间（CST，UTC+8）
            let beijing_time = utc_time.with_timezone(&chrono::FixedOffset::east(8 * 3600));

            println!("当前时间: {}", now);
            println!("下一次执行时间: {}", beijing_time);
            println!("下一次执行时间: {}", beijing_time.timestamp());
        } else {
            panic!("无法计算下一次执行时间");
        }
    }

    #[test]
    fn test_invalid_cron_expr() {
        // 测试无效的 cron 表达式
        let cron_expr = "invalid cron expression";

        // 检查返回值是否为 None
        assert!(get_next_time(cron_expr).is_none());
    }

    #[test]
    fn test_get_next_time_specific_case() {
        // 测试一个具体的 cron 表达式，假设当前时间是 2024-11-05 12:00:00
        let cron_expr = "0 0 12 * * *"; // 每天中午12点执行

        // 设定一个固定的时间，模拟当前时间
        let fixed_now = NaiveDateTime::parse_from_str("2024-11-05 11:59:00", "%Y-%m-%d %H:%M:%S")
            .expect("时间解析失败");
        let fixed_now = chrono::DateTime::<Utc>::from_utc(fixed_now, Utc);

        // 获取下一次执行时间
        if let Some(next_time) = get_next_time(cron_expr) {
            let next_time_naive = NaiveDateTime::from_timestamp(next_time, 0);

            // 断言下一次执行时间应该是 2024-11-05 12:00:00
            let expected_next_time =
                NaiveDateTime::parse_from_str("2024-11-05 12:00:00", "%Y-%m-%d %H:%M:%S")
                    .expect("时间解析失败");
            assert_eq!(next_time_naive, expected_next_time);
        } else {
            panic!("无法计算下一次执行时间");
        }
    }
}

pub async fn calc_handler_mq(
    influxdb: InfluxConfig,
    guard: &RedisOp,
    rabbit_conn: &Connection,
    channel1: &Channel,
    script_waring_collection: String,
    mongo_dbmanager: &MongoDBManager,
) {
    let mut consumer = channel1
        .basic_consume(
            "calc_queue",
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    trace!("rmq consumer connected, waiting for messages");
    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                trace!("received msg: {:?}", delivery);

                let result = String::from_utf8(delivery.data).unwrap();

                calc_handler(
                    result,
                    guard,
                    influxdb.bucket.clone().unwrap(),
                    influxdb.host.clone().unwrap().as_str(),
                    influxdb.port.clone().unwrap(),
                    influxdb.org.clone().unwrap().as_str(),
                    influxdb.token.clone().unwrap().as_str(),
                    &mongo_dbmanager.clone(),
                    script_waring_collection.clone(),
                )
                .await;

                match channel1
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                {
                    Ok(_) => {
                        trace!("消息已成功确认。");
                    }
                    Err(e) => {
                        error!("确认消息时发生错误: {}", e);
                        // 这里可以添加进一步的错误处理逻辑
                    }
                }
            }
            Err(err) => {
                error!("Error receiving message: {:?}", err);
            }
        }
    }
}
