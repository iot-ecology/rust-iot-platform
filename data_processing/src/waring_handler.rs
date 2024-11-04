use chrono::Utc;
use common_lib::models::DataRowList;
use common_lib::redis_handler::get_redis_instance;
use log::{debug, error, info};
use std::collections::HashMap;

pub async fn handler_waring_once(
    dt: DataRowList,
    waring_collection: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let device_uid_string = &*dt.device_uid;
    let iden_code = &*dt.identification_code;
    let push_time = dt.time;
    let mapping = get_mapping_signal_waring_config(device_uid_string, iden_code)
        .await
        .unwrap();
    let now = Utc::now();
    let guard = get_mongo().await.unwrap();

    for x in dt.data {
        debug!(" x  :{:?}", x);

        let x1 = mapping.get(x.name.as_str()).unwrap();
        debug!("x1 = {:?}", x1);

        let floatValue = x.value.parse::<f64>().unwrap();

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
                    document.insert("signal_name".to_string(), serde_json::json!(x.name));
                    document.insert("signal_id".to_string(), serde_json::json!(config.signal_id));
                    document.insert("value".to_string(), serde_json::json!(floatValue));
                    document.insert("rule_id".to_string(), serde_json::json!(config.id));
                    document.insert(
                        "insert_time".to_string(),
                        serde_json::json!(now.timestamp()),
                    );
                    document.insert("up_time".to_string(), serde_json::json!(push_time));
                    info!("命中报警 in_or_out = 1");

                    guard
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
                    document.insert("rule_id".to_string(), serde_json::json!(device_uid_string));
                    document.insert(
                        "insert_time".to_string(),
                        serde_json::json!(device_uid_string),
                    );
                    document.insert("up_time".to_string(), serde_json::json!(device_uid_string));
                    info!("命中报警 in_or_out = 0");
                    guard
                        .insert_document(name.as_str(), document)
                        .await
                        .unwrap();
                }
            }
            // todo: message template
        }
    }
    Ok(())
}

fn calc_collection_name(prefix: &str, id: i32) -> String {
    let string = format!("{}_{}", prefix, id % 100);
    return string;
}

async fn get_mapping_signal_waring_config(
    device_uid_string: &str,
    iden_code: &str,
) -> Result<HashMap<String, Vec<SignalWaringConfig>>, Box<dyn std::error::Error>> {
    let guard = get_redis_instance().await.unwrap();
    let key = format!("signal:{}:{}", device_uid_string, iden_code);
    debug!("key = {}", key);
    let vec = guard.get_list_all(key.as_str()).await.unwrap();
    let mut mapping: HashMap<String, Vec<SignalWaringConfig>> = HashMap::new();
    for str_signal in vec {
        let signal: Signal = match serde_json::from_str(&str_signal) {
            Ok(s) => s,
            Err(_) => continue, // 跳过反序列化失败的信号
        };

        let key_warning = format!("waring:{}", signal.id);
        debug!("key_warning = {}", key_warning);
        let warnings = guard.get_list_all(key_warning.as_str()).await.unwrap();

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
            swc.unit = signal.unit.clone();
            swcs.push(swc);
        }
        debug!("signal.name = {}", signal.name);

        mapping.insert(signal.name, swcs);
    }
    Ok(mapping)
}

#[derive(Deserialize, Serialize, Debug)]
struct Signal {
    #[serde(rename = "ID")] // 在序列化时使用 "ID"
    id: i32,
    name: String,
    unit: Option<String>,
}

use crate::storage_handler::storage_data_row;
use common_lib::mongo_utils::get_mongo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SignalWaringConfig {
    #[serde(rename = "signal_id")]
    signal_id: i32, // 信号表的外键ID
    #[serde(rename = "min")]
    min: f64, // 范围, 小值
    #[serde(rename = "max")]
    max: f64, // 范围, 大值
    #[serde(rename = "in_or_out")]
    in_or_out: i32, // 1 范围内报警 0 范围外报警
    #[serde(rename = "unit")]
    unit: Option<String>, // 单位
    #[serde(rename = "ID")]
    id: i32, // ID
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_lib::init_logger;
    use common_lib::models::DataRow;
    use common_lib::mongo_utils::init_mongo;
    use common_lib::protocol_config::{get_config, read_config};
    use common_lib::rabbit_utils::init_rabbitmq_with_config;
    use common_lib::redis_handler::init_redis;
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
        let now = Utc::now();
        let dt = DataRowList {
            time: now.timestamp(),
            device_uid: "1".to_string(),
            identification_code: "1".to_string(),
            data: vec![DataRow {
                name: "信号-199".to_string(),
                value: "2".to_string(),
            }],
            nc: "1".to_string(),
            protocol: Some("MQTT".to_string()),
        };

        if let Err(e) = handler_waring_once(dt, mongo_config.waring_collection.unwrap()).await {
            log::error!("Failed to store data row: {:?}", e);
        }
    }
}
