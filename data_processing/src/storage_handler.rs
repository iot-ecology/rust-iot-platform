use common_lib::models::DataRowList;

pub async fn storage_data_row(dt: DataRowList, protocol: &str) {
    let map = get_mqtt_client_signal(&*dt.device_uid, &*dt.identification_code)
        .await
        .unwrap();

    // calc_bucket_name()
}

use common_lib::redis_handler::get_redis_instance;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub cache_size: usize,
    #[serde(rename = "ID")] // 在序列化时使用 "ID"
    pub id: i64,
    pub r#type: String,
}

#[derive(Debug)]
pub struct SignalMapping {
    pub cache_size: usize,
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
        let signal: Signal = match serde_json::from_str(str_signal.as_str()) {
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

fn gen_measurement(dt: &DataRowList, protocol: &str) -> String {
    format!("{}_{}_{}", protocol, dt.device_uid, dt.identification_code)
}
fn calc_bucket_name(prefix: &str, protocol: &str, id: u32) -> String {
    format!("{}_{}_{}", prefix, protocol, id % 100)
}
#[cfg(test)]
mod tests {
    use super::*;
    use common_lib::init_logger;
    use common_lib::protocol_config::{get_config, read_config};
    use common_lib::rabbit_utils::init_rabbitmq_with_config;
    use common_lib::redis_handler::init_redis;
    use log::info;

    #[tokio::test]
    async fn test_get_mqtt_client_signal() {
        let addr = "127.0.0.1:5683";

        init_logger();
        let result = read_config("app-local.yml").await.unwrap();
        let config = get_config().await.unwrap();
        // let node_info_name = config.node_info.name.clone();

        let redis_config = config.redis_config.clone();
        init_redis(redis_config).await.unwrap();
        init_rabbitmq_with_config(config.mq_config.clone())
            .await
            .unwrap();

        // 调用 get_mqtt_client_signal 函数
        let result = get_mqtt_client_signal("1", "1").await.unwrap();

        info!("{:?}", result);
    }
}
