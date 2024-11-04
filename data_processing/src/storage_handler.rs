use chrono::Utc;
use common_lib::influxdb_utils::InfluxDBManager;
use common_lib::models::{DataRowList, DataValue};
use common_lib::protocol_config::get_config;
use common_lib::redis_handler::get_redis_instance;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;

use log::{error, info};

pub async fn storage_data_row(
    dt: DataRowList,
    protocol: &str,
    host: &str,
    port: u16,
    org: &str,
    token: &str,
    bucket_pre: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let device_uid_string = &*dt.device_uid;
    let iden_code = &*dt.identification_code;
    let push_time = dt.time;

    // 获取 MQTT 客户端信号
    let map = match get_mqtt_client_signal(device_uid_string, iden_code).await {
        Ok(map) => map,
        Err(e) => {
            error!("Failed to get MQTT client signal: {:?}", e);
            return Err(e);
        }
    };

    // 解析设备 UID
    let device_uid: u32 = match device_uid_string.parse::<u32>() {
        Ok(uid) => uid,
        Err(_) => {
            error!("Not a valid u32 number: {}", device_uid_string);
            return Err("Not a valid u32 number".into());
        }
    };

    let bucket_name = calc_bucket_name(bucket_pre, protocol, device_uid);

    info!("bucket_name: {}", bucket_name);
    // 创建 InfluxDB 管理器
    let db_manager = InfluxDBManager::new(host, port, org, token);

    let now = Utc::now();
    let measur = gen_measurement(device_uid_string, iden_code, protocol);

    let mut insert_dt = HashMap::new();
    let now_timestamp = now.timestamp();

    insert_dt.insert(
        "storage_time".to_string(),
        DataValue::Integer(now_timestamp),
    );
    insert_dt.insert("push_time".to_string(), DataValue::Integer(push_time));
    insert_dt.insert(
        "time-sub".to_string(),
        DataValue::Integer(now_timestamp - push_time),
    );

    for x in dt.data {
        let data_value = x.value;

        let x1 = match map.get(x.name.as_str()) {
            Some(mapping) => mapping,
            None => {
                error!("Signal not found in mapping: {}", x.name);
                continue; // 或者处理缺失的信号
            }
        };

        if x1.numb {
            let float_num: f64 = match data_value.parse() {
                Ok(num) => num,
                Err(_) => {
                    error!("Failed to parse string to float: {}", data_value);
                    continue; // 或者处理解析错误
                }
            };
            insert_dt.insert(x1.id.to_string(), DataValue::Float(float_num));
        } else {
            insert_dt.insert(x1.id.to_string(), DataValue::Text(data_value.clone()));
        }

        let guard = get_redis_instance().await.unwrap();
        let key = format!(
            "signal_delay_warning:{}:{}:{}",
            device_uid, iden_code, x1.id
        );
        if x1.cache_size > 0 {
            // i := x1.cache_size + 1 - currentSize

            info!("signal_delay_warning key = {}", key);

            let currentSize = guard.get_zset_length(key.as_str()).await.unwrap();

            if currentSize >= x1.cache_size {
                let i = x1.cache_size + 1 - currentSize;
                if i == 1 {
                    guard.delete_first_zset_member(key.as_str()).await.unwrap();
                    guard
                        .add_zset(key.as_str(), data_value.as_str(), now_timestamp as f64)
                        .await
                        .unwrap();
                } else {
                }
            } else {
                guard
                    .add_zset(key.as_str(), data_value.as_str(), now_timestamp as f64)
                    .await
                    .unwrap();
            }
        }
    }

    // 写入数据
    if let Err(e) = db_manager
        .write(insert_dt, measur.as_str(), bucket_name.as_str())
        .await
    {
        error!("Failed to write data to InfluxDB: {:?}", e);
        return Err(e);
    }

    set_push_time(protocol, iden_code, device_uid_string, now_timestamp).await;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub cache_size: u64,
    #[serde(rename = "ID")] // 在序列化时使用 "ID"
    pub id: i64,
    pub r#type: String,
}

#[derive(Debug)]
pub struct SignalMapping {
    pub cache_size: u64,
    pub id: i64,
    pub numb: bool,
}

pub async fn get_mqtt_client_signal(
    mqtt_client_id: &str,
    identification_code: &str,
) -> Result<HashMap<String, SignalMapping>, Box<dyn std::error::Error>> {
    let redis = get_redis_instance().await?;
    let key = format!("signal:{}:{}", mqtt_client_id, identification_code);

    // 从 Redis 获取列表
    let result: Vec<String> = redis.get_list_all(key.as_str()).await?;

    let mut mapping = HashMap::new();

    for str_signal in result {
        let signal: Signal = match from_str(str_signal.as_str()) {
            Ok(signal) => signal,
            Err(err) => {
                // 处理反序列化错误，您可以记录日志或采取其他措施
                log::error!("Failed to deserialize signal: {:?}", err);
                continue; // 跳过当前信号
            }
        };

        mapping.insert(
            signal.name.clone(),
            SignalMapping {
                cache_size: signal.cache_size,
                id: signal.id,
                numb: signal.r#type.eq_ignore_ascii_case("数字"),
            },
        );
    }

    Ok(mapping)
}

fn gen_measurement(device_uid: &str, identification_code: &str, protocol: &str) -> String {
    format!("{}_{}_{}", protocol, device_uid, identification_code)
}
fn calc_bucket_name(prefix: &str, protocol: &str, id: u32) -> String {
    format!("{}_{}_{}", prefix, protocol, id % 100)
}
#[cfg(test)]
mod tests {
    use super::*;
    use common_lib::init_logger;
    use common_lib::models::DataRow;
    use common_lib::protocol_config::{get_config, read_config};
    use common_lib::rabbit_utils::init_rabbitmq_with_config;
    use common_lib::redis_handler::init_redis;
    use log::info;

    #[tokio::test]
    async fn test_storage() {
        init_logger();

        let result = read_config("app-local.yml").await.unwrap();
        let config = get_config().await.unwrap();

        let redis_config = config.redis_config.clone();
        let influxdb = config.influx_config.clone().unwrap();
        init_redis(redis_config).await.unwrap();
        init_rabbitmq_with_config(config.mq_config.clone())
            .await
            .unwrap();

        let now = Utc::now();
        let dt = DataRowList {
            time: now.timestamp(),
            device_uid: "1".to_string(),
            identification_code: "1".to_string(),
            data: vec![DataRow {
                name: "信号-31".to_string(),
                value: "2".to_string(),
            }],
            nc: "1".to_string(),
            protocol: Some("MQTT".to_string()),
        };

        if let Err(e) = storage_data_row(
            dt,
            "MQTT",
            influxdb.host.unwrap().as_str(),
            influxdb.port.unwrap(),
            influxdb.org.unwrap().as_str(),
            influxdb.token.unwrap().as_str(),
            influxdb.bucket.unwrap().as_str(),
        )
        .await
        {
            log::error!("Failed to store data row: {:?}", e);
        }
    }
}

pub async fn set_push_time(
    protocol: &str,
    identification_code: &str,
    device_uid: &str,
    time_from_unix: i64,
) {
    let pre_key = "storage_time";
    let guard = get_redis_instance().await.unwrap();
    let key = format!(
        "{}:{}:{}:{}",
        pre_key, protocol, device_uid, identification_code
    );

    guard
        .set_string(key.as_str(), time_from_unix.to_string().as_str())
        .await
        .unwrap();
}
