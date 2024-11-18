use chrono::Utc;
use common_lib::config::{get_config, Config, InfluxConfig};
use common_lib::influxdb_utils::InfluxDBManager;
use common_lib::models::{DataRowList, DataValue, MQTTMessage, Signal, SignalMapping};
use common_lib::rabbit_utils::RabbitMQ;
use common_lib::redis_handler::{get_redis_instance, RedisWrapper};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::ut::calc_bucket_name;
use futures_util::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection};
use log::{error, info};
use quick_js::{Context, ContextError};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub async fn storage_data_row(
    dt: DataRowList,
    protocol: &str,
    host: &str,
    port: u16,
    org: &str,
    token: &str,
    bucket_pre: &str,
    redis: &RedisOp,
) -> Result<(), Box<dyn std::error::Error>> {
    let device_uid_string = &*dt.DeviceUid;
    let iden_code = &*dt.IdentificationCode;
    let push_time = dt.Time;

    // 获取 MQTT 客户端信号
    let map = match get_mqtt_client_signal(device_uid_string, iden_code, redis) {
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

    let now = common_lib::time_utils::local_to_utc();
    let measur = calc_measurement(device_uid_string, iden_code, protocol);

    let mut insert_dt = HashMap::new();
    let now_timestamp = now;

    insert_dt.insert(
        "storage_time".to_string(),
        DataValue::Integer(now_timestamp),
    );
    insert_dt.insert("push_time".to_string(), DataValue::Integer(push_time));
    insert_dt.insert(
        "time-sub".to_string(),
        DataValue::Integer(now_timestamp - push_time),
    );

    for x in dt.DataRows {
        let data_value = x.Value;

        let x1 = match map.get(x.Name.as_str()) {
            Some(mapping) => mapping,
            None => {
                error!("Signal not found in mapping: {}", x.Name);
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

        let key = format!(
            "signal_delay_warning:{}:{}:{}",
            device_uid, iden_code, x1.id
        );
        if x1.cache_size > 0 {
            // i := x1.cache_size + 1 - currentSize

            info!("signal_delay_warning key = {}", key);

            let currentSize = redis.get_zset_length(key.as_str()).unwrap();

            if currentSize >= x1.cache_size {
                let i = x1.cache_size + 1 - currentSize;
                if i == 1 {
                    redis.delete_first_zset_member(key.as_str()).unwrap();
                    redis
                        .add_zset(key.as_str(), data_value.as_str(), now_timestamp as f64)
                        .unwrap();
                } else {
                }
            } else {
                redis
                    .add_zset(key.as_str(), data_value.as_str(), now_timestamp as f64)
                    .unwrap();
            }
        }
    }
    info!("insert_dt.len() = {}", insert_dt.len());

    info!("measur.as_str() = {}", measur.as_str());

    // 写入数据
    if let Err(e) = db_manager
        .write(insert_dt, measur.as_str(), bucket_name.as_str())
        .await
    {
        error!("Failed to write data to InfluxDB: {:?}", e);
        return Err(e);
    }

    set_push_time(protocol, iden_code, device_uid_string, now_timestamp, redis);

    Ok(())
}

pub fn get_mqtt_client_signal(
    mqtt_client_id: &str,
    identification_code: &str,
    redis: &RedisOp,
) -> Result<HashMap<String, SignalMapping>, Box<dyn std::error::Error>> {
    let key = format!("signal:{}:{}", mqtt_client_id, identification_code);

    // 从 Redis 获取列表
    let result = redis.get_list_all(key.as_str()).unwrap();

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

pub fn calc_measurement(device_uid: &str, identification_code: &str, protocol: &str) -> String {
    format!("{}_{}_{}", protocol, device_uid, identification_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_lib::config::{get_config, read_config, read_config_tb};
    use common_lib::init_logger;
    use common_lib::models::DataRow;
    use common_lib::rabbit_utils::init_rabbitmq_with_config;
    use common_lib::redis_handler::init_redis;
    use common_lib::redis_pool_utils::create_redis_pool_from_config;
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

        let now = common_lib::time_utils::local_to_utc();
        let dt = DataRowList {
            Time: now,
            DeviceUid: "1".to_string(),
            IdentificationCode: "1".to_string(),
            DataRows: vec![DataRow {
                Name: "信号-31".to_string(),
                Value: "2".to_string(),
            }],
            Nc: "1".to_string(),
            Protocol: Some("MQTT".to_string()),
        };
        let config1 = read_config_tb("app-local.yml");
        let pool = create_redis_pool_from_config(&config1.redis_config);

        let redisOp = RedisOp { pool };
        if let Err(e) = storage_data_row(
            dt,
            "MQTT",
            influxdb.host.unwrap().as_str(),
            influxdb.port.unwrap(),
            influxdb.org.unwrap().as_str(),
            influxdb.token.unwrap().as_str(),
            influxdb.bucket.unwrap().as_str(),
            &redisOp,
        )
        .await
        {
            log::error!("Failed to store data row: {:?}", e);
        }
    }
}

pub fn set_push_time(
    protocol: &str,
    identification_code: &str,
    device_uid: &str,
    time_from_unix: i64,
    redis: &RedisOp,
) {
    let pre_key = "storage_time";
    let key = format!(
        "{}:{}:{}:{}",
        pre_key, protocol, device_uid, identification_code
    );

    redis
        .set_string(key.as_str(), time_from_unix.to_string().as_str())
        .unwrap();
}

async fn handler_data_storage_string(
    result: String,
    jsc: Context,
    config: InfluxConfig,
    redis: &RedisOp,
    rabbit_conn: &Connection,
) -> Result<(), Box<dyn Error>> {
    info!("message : {:?}", result);

    // 尝试反序列化 MQTT 消息
    let mqtt_message: MQTTMessage = serde_json::from_str(&result)?;

    // 获取存储的脚本
    let option = redis
        .get_hash("mqtt_script", mqtt_message.mqtt_client_id.as_str())
        .unwrap();

    if let Some(string) = option {
        // 在这里创建 JavaScript 上下文
        jsc.eval(&string)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // 调用 main 函数
        let value = jsc
            .call_function("main", [mqtt_message.message])
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // 定义并调用 JavaScript 代码
        let js_code_2 = r#"
        function main2(data) {
            return JSON.stringify(data);
        }"#;
        jsc.eval(js_code_2)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let value2 = jsc
            .call_function("main2", [value])
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let x = value2.as_str().unwrap_or("");

        info!("Java Script Result = {:?}", x);
        let dt: Vec<DataRowList> = from_str(&x).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        info!("{:?}", dt);

        // 存储数据行
        for mut data_row in dt {
            data_row.Protocol = Some("MQTT".to_string());
            storage_data_row(
                data_row,
                "MQTT",
                config.host.clone().unwrap().as_str(),
                config.port.clone().unwrap(),
                config.org.clone().unwrap().as_str(),
                config.token.clone().unwrap().as_str(),
                config.bucket.clone().unwrap().as_str(),
                redis,
            )
            .await
            .expect("storage_data_row error");
        }

        // 创建 RabbitMQ 通道
        let rabbit_channel = rabbit_conn
            .create_channel()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // 消息推送
        for queue in ["waring_handler", "waring_delay_handler", "transmit_handler"].iter() {
            rabbit_channel
                .basic_publish(
                    "",
                    *queue,
                    BasicPublishOptions::default(),
                    x.clone().as_bytes(),
                    BasicProperties::default(),
                )
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        }

        // fixme: 处理最后推送时间（如果需要的话）
    } else {
        info!(
            "未找到脚本 for mqtt_client_id: {}",
            mqtt_message.mqtt_client_id
        );
    }

    Ok(())
}

pub async fn pre_handler(
    guard1: &Config,
    guard: &RedisOp,
    rabbit_conn: &Connection,
    channel1: &Channel,
) {
    let mut consumer = channel1
        .basic_consume(
            "pre_handler",
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

                match handler_data_storage_string(
                    result,
                    Context::new().unwrap(),
                    guard1.influx_config.clone().unwrap(),
                    guard,
                    rabbit_conn,
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
