mod ctr;
mod mqtt_async_sample;
mod mqtt_sync_sample;

use crate::ctr::GetNoUseMqttConfig;
use crate::ctr::GetUseMqttConfig;
use crate::ctr::HttpBeat;
use crate::ctr::NodeList;
use crate::ctr::NodeUsingStatus;
use crate::ctr::PubCreateMqttClientHttp;
use crate::ctr::PubRemoveMqttClient;
use crate::ctr::RemoveMqttClient;
use crate::ctr::{addNoUseConfig, create_mqtt_client_http};
use common_lib::config::{get_config, read_config, read_config_tb};
use common_lib::models::MqttConfig;
use common_lib::rabbit_utils::init_rabbitmq_with_config;
use common_lib::redis_handler::init_redis;
use common_lib::redis_pool_utils::{create_redis_pool_from_config, RedisOp};
use log::error;
use r2d2_redis::redis::RedisError;
use rocket::{launch, routes};
use serde_json::from_str;
use tokio::runtime::Runtime;

#[launch]
fn rocket() -> _ {
    common_lib::init_logger(); // 初始化日志记录
    crate::mqtt_async_sample::init_mqtt_map();
    // 创建异步运行时
    let rt = Runtime::new().unwrap();
    let config1 = read_config_tb("app-local.yml");

    let pool = create_redis_pool_from_config(&config1.redis_config);

    let redis_op = RedisOp { pool };
    beforeStart(&redis_op, &config1.clone());

    // 构建并启动 Rocket 应用
    rocket::build()
        .manage(redis_op)
        .manage(config1.clone())
        .configure(rocket::Config {
            port: config1.node_info.port,
            ..Default::default()
        })
        .mount(
            "/",
            routes![
                HttpBeat,
                create_mqtt_client_http,
                NodeList,
                NodeUsingStatus,
                GetUseMqttConfig,
                GetNoUseMqttConfig,
                RemoveMqttClient,
                PubCreateMqttClientHttp,
                PubRemoveMqttClient,
            ],
        )
}

fn beforeStart(redis_op: &RedisOp, config: &common_lib::config::Config) {}

pub fn HandlerOffNode(node_name: String, redis_op: &RedisOp) {
    let vec = GetBindClientId(node_name.clone(), redis_op);
    for x in vec {
        if let Some(cf) = GetUseConfig(x, redis_op) {
            if let Ok(mqtt_config) = from_str::<MqttConfig>(cf.as_str()) {
                RemoveBindNode(node_name.clone(), mqtt_config.client_id, redis_op);
            } else {
                eprintln!("解析 MQTT 配置失败");
            }
        }
    }
}

fn CheckMqttConfigIsUsingAndMove(node_name: String, redis_op: &RedisOp) {}
pub fn RemoveBindNode(node_name: String, mqtt_client_id: String, redis_op: &RedisOp) {
    redis_op.delete_set_member(
        format!("node_bind:{}", node_name).as_str(),
        mqtt_client_id.as_str(),
    );

    let option = GetUseConfig(mqtt_client_id, redis_op);
    match option {
        None => {}
        Some(str) => {
            let mqtt_config: MqttConfig = from_str::<MqttConfig>(str.as_str()).unwrap();
            RemoveUseConfig(mqtt_config.client_id.clone(), redis_op);
            addNoUseConfig(&mqtt_config, redis_op);
        }
    }
}
pub fn GetUseConfig(client_id: String, redis_op: &RedisOp) -> Option<String> {
    let result = redis_op
        .get_hash("mqtt_config:use", client_id.as_str())
        .unwrap();
    result
}

pub fn RemoveUseConfig(client_id: String, redis_op: &RedisOp) {
    redis_op
        .delete_hash_field("mqtt_config:use", client_id.as_str())
        .unwrap();
}

pub fn GetBindClientId(nodeName: String, redis_op: &RedisOp) -> Vec<String> {
    let key = format!("node_bind:{}", nodeName);
    let result = redis_op.get_set(key.as_str());
    match result {
        Ok(res) => {
            return res;
        }
        Err(e) => {
            error!("{}", e);
            return vec![];
        }
    }
}
