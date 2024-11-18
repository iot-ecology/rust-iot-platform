use chrono::Utc;
use common_lib::models::{DataRowList, MQTTMessage, Signal, SignalWaringConfig};
use common_lib::redis_handler::RedisWrapper;
use futures_util::StreamExt;
use lapin::options::BasicAckOptions;
use lapin::options::{BasicConsumeOptions, BasicPublishOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection};
use log::{debug, error, info};
use quick_js::{Context, ContextError};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub async fn handler_waring_once(
    dt: DataRowList,
    waring_collection: String,
    redis: &RedisOp,
    mongo_dbmanager: &MongoDBManager,
) -> Result<(), Box<dyn std::error::Error>> {
    let device_uid_string = &*dt.DeviceUid;
    let iden_code = &*dt.IdentificationCode;
    let push_time = dt.Time;
    let mapping = get_mapping_signal_waring_config(device_uid_string, iden_code, redis).unwrap();
    let now = common_lib::time_utils::local_to_utc();

    for x in dt.DataRows {
        debug!(" x  :{:?}", x);

        let xdata = mapping.get(x.Name.as_str());
        if xdata.is_some() {
            let x1 = xdata.unwrap();
            debug!("x1 = {:?}", x1);

            let floatValue = x.Value.parse::<f64>().unwrap();

            for config in x1 {
                let name = calc_collection_name(waring_collection.as_str(), config.id);
                if config.in_or_out == 1 {
                    if (config.min <= floatValue && floatValue <= config.max) {
                        //     范围内
                        let mut document = HashMap::new();
                        document.insert(
                            "device_uid".to_string(),
                            serde_json::json!(device_uid_string),
                        );
                        document.insert("signal_name".to_string(), serde_json::json!(x.Name));
                        document
                            .insert("signal_id".to_string(), serde_json::json!(config.signal_id));
                        document.insert("value".to_string(), serde_json::json!(floatValue));
                        document.insert("rule_id".to_string(), serde_json::json!(config.id));
                        document.insert("insert_time".to_string(), serde_json::json!(now));
                        document.insert("up_time".to_string(), serde_json::json!(push_time));
                        info!("命中报警 in_or_out = 1");

                        mongo_dbmanager
                            .insert_document(name.as_str(), document)
                            .await
                            .unwrap();
                    }
                } else {
                    if (floatValue < config.min || floatValue > config.max) {
                        //     范围外
                        let mut document = HashMap::new();
                        document.insert(
                            "device_uid".to_string(),
                            serde_json::json!(device_uid_string),
                        );
                        document.insert(
                            "signal_name".to_string(),
                            serde_json::json!(device_uid_string),
                        );
                        document.insert(
                            "signal_id".to_string(),
                            serde_json::json!(device_uid_string),
                        );
                        document.insert("value".to_string(), serde_json::json!(device_uid_string));
                        document
                            .insert("rule_id".to_string(), serde_json::json!(device_uid_string));
                        document.insert(
                            "insert_time".to_string(),
                            serde_json::json!(device_uid_string),
                        );
                        document
                            .insert("up_time".to_string(), serde_json::json!(device_uid_string));
                        info!("命中报警 in_or_out = 0");
                        mongo_dbmanager
                            .create_collection(name.as_str())
                            .await
                            .unwrap();

                        mongo_dbmanager
                            .insert_document(name.as_str(), document)
                            .await
                            .unwrap();
                    }
                }
                // todo: message template
            }
        }
    }
    Ok(())
}


fn get_mapping_signal_waring_config(
    device_uid_string: &str,
    iden_code: &str,
    redis_wrapper: &RedisOp,
) -> Result<HashMap<String, Vec<SignalWaringConfig>>, Box<dyn std::error::Error>> {
    let key = format!("signal:{}:{}", device_uid_string, iden_code);
    debug!("key = {}", key);
    let vec = redis_wrapper.get_list_all(key.as_str()).unwrap();
    let mut mapping: HashMap<String, Vec<SignalWaringConfig>> = HashMap::new();
    for str_signal in vec {
        let signal: Signal = match serde_json::from_str(&str_signal) {
            Ok(s) => s,
            Err(_) => continue, // 跳过反序列化失败的信号
        };

        let key_warning = format!("waring:{}", signal.id);
        debug!("key_warning = {}", key_warning);
        let warnings = redis_wrapper.get_list_all(key_warning.as_str()).unwrap();

        let mut swcs: Vec<SignalWaringConfig> = vec![];
        for sw in warnings {
            debug!("sw = {:?}", sw);
            let mut swc: SignalWaringConfig = match serde_json::from_str(&sw) {
                Ok(w) => w,
                Err(e) => {
                    error!("swc 反序列化失败");
                    error!("e = {}", e);

                    continue;
                } // 跳过反序列化失败的警告配置
            };
            swcs.push(swc);
        }
        debug!("signal.name = {}", signal.name);

        mapping.insert(signal.name, swcs);
    }
    Ok(mapping)
}

pub async fn handler_waring_string(
    result: String,
    jsc: Context,
    config: InfluxConfig,
    redis: &RedisOp,
    rabbit_conn: &Connection,
    waring_collection: String,
    mongo_dbmanager: &MongoDBManager,
) -> Result<(), Box<dyn Error>> {
    info!("message : {:?}", result);

    // 尝试反序列化 MQTT 消息
    let dt: Vec<DataRowList> = serde_json::from_str(&result)?;

    for x in dt {
        handler_waring_once(x, waring_collection.clone(), redis, mongo_dbmanager)
            .await
            .unwrap();
    }

    Ok(())
}

use crate::storage_handler::storage_data_row;
use common_lib::config::InfluxConfig;
use common_lib::mongo_utils::{get_mongo, MongoDBManager};
use common_lib::redis_pool_utils::RedisOp;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use common_lib::ut::calc_collection_name;

#[cfg(test)]
mod tests {
    use super::*;
    use common_lib::config::{get_config, read_config, read_config_tb};
    use common_lib::init_logger;
    use common_lib::models::DataRow;
    use common_lib::mongo_utils::init_mongo;
    use common_lib::rabbit_utils::init_rabbitmq_with_config;
    use common_lib::redis_handler::{get_redis_instance, init_redis};
    use common_lib::redis_pool_utils::create_redis_pool_from_config;
    use log::debug;

    #[tokio::test]
    async fn test_storage() {
        init_logger();

        let result = read_config("app-local.yml").await.unwrap();
        let config = get_config().await.unwrap();

        let redis_config = config.redis_config.clone();
        let mongo_config = config.mongo_config.clone().unwrap();

        let influxdb = config.influx_config.clone().unwrap();
        init_redis(redis_config).await.unwrap();
        init_rabbitmq_with_config(config.mq_config.clone())
            .await
            .unwrap();

        init_mongo(mongo_config.clone()).await.unwrap();
        let now = common_lib::time_utils::local_to_utc();
        let dt = DataRowList {
            Time: now,
            DeviceUid: "1".to_string(),
            IdentificationCode: "1".to_string(),
            DataRows: vec![DataRow {
                Name: "信号-199".to_string(),
                Value: "2".to_string(),
            }],
            Nc: "1".to_string(),
            Protocol: Some("MQTT".to_string()),
        };

        let wrapper = get_redis_instance().await.unwrap().clone();
        let guard = get_mongo().await.unwrap().clone();
        let config1 = read_config_tb("app-local.yml");
        let pool = create_redis_pool_from_config(&config1.redis_config);

        let redisOp = RedisOp { pool };
        if let Err(e) = handler_waring_once(
            dt,
            mongo_config.waring_collection.unwrap(),
            &redisOp,
            &guard,
        )
        .await
        {
            log::error!("Failed to store data row: {:?}", e);
        }
    }
}

pub async fn waring_handler(
    influx_config: InfluxConfig,
    guard: &RedisOp,
    rabbit_conn: &Connection,
    channel1: &Channel,
    waring_collection: String,
    mongo_dbmanager: &MongoDBManager,
) {
    let mut consumer = channel1
        .basic_consume(
            "waring_handler",
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    info!("rmq consumer connected, waiting for messages");
    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                info!("received msg: {:?}", delivery);

                let result = String::from_utf8(delivery.data).unwrap();

                match handler_waring_string(
                    result,
                    Context::new().unwrap(),
                    influx_config.clone(),
                    guard,
                    rabbit_conn,
                    waring_collection.clone(),
                    mongo_dbmanager,
                )
                .await
                {
                    Ok(_) => {
                        info!("msg processed");
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                };

                match channel1
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                {
                    Ok(_) => {
                        info!("消息已成功确认。");
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
