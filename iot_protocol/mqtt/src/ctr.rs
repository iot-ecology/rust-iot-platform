use crate::mqtt_async_sample::{create_client, event_loop};
use common_lib::config::{Config, NodeInfo};
use common_lib::models::MqttConfig;
use common_lib::redis_pool_utils::RedisOp;
use log::{error, info};
use r2d2::Pool;
use r2d2_redis::redis::RedisError;
use r2d2_redis::RedisConnectionManager;
use rocket::fairing::AdHoc;
use rocket::http::{Method, Status};
use rocket::serde::json::Json;
use rocket::tokio::time::Duration;
use rocket::yansi::Paint;
use rocket::{get, post, State};
use rocket::{Request, Response};
use rumqttc::{Client, ConnectionError, Event, Incoming, MqttOptions, QoS};
use serde_json::json;
use std::future::poll_fn;
use std::string::String;
use tokio::task;

#[get("/beat")]
pub async fn HttpBeat(pool: &rocket::State<RedisOp>) -> rocket::response::status::Custom<&str> {
    return rocket::response::status::Custom(Status::Ok, "ok");
}

#[post("/create_mqtt", format = "json", data = "<mqtt_config>")]
pub async fn create_mqtt_client_http(
    redis_op: &State<RedisOp>,
    config: &State<Config>,
    mqtt_config: Json<MqttConfig>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    info!("mqtt_config = {:?}", mqtt_config);
    info!("config = {:?}", config);

    let key = format!("mqtt_create:{}", mqtt_config.client_id);
    let x = redis_op.acquire_lock(&key, &key, 100).unwrap();
    if x {
        if check_has_config(mqtt_config.client_id.clone(), redis_op) {
            info!("当前客户端已存在");

            let response = json!({
                "status": 400,
                "message": "已经存在客户端id"
            });
            return rocket::response::status::Custom(Status::Ok, Json(response));
        } else {
            let usz = create_mqtt_client(&mqtt_config, &redis_op, &config.node_info).await;

            if usz == -1 {
                let response = json!({
                    "status": 400,
                    "message": "达到最大客户端数量"
                });
                redis_op.release_lock(&key, &key).unwrap();
                return rocket::response::status::Custom(Status::Ok, Json(response));
            } else if usz == -2 {
                let response = json!({
                    "status": 400,
                    "message": "MQTT客户端配置异常"
                });
                redis_op.release_lock(&key, &key).unwrap();
                return rocket::response::status::Custom(Status::Ok, Json(response));
            } else {
                AddNoUseConfig(&mqtt_config, redis_op);
                BindNode(&mqtt_config, config.node_info.name.clone(), redis_op);
                let response = json!({
                    "status": 200,
                    "message": "创建成功"
                });
                redis_op.release_lock(&key, &key).unwrap();
                return rocket::response::status::Custom(Status::Ok, Json(response));
            }
        }
    } else {
        let response = json!({
            "status": 400,
            "message": "上锁异常"
        });
        rocket::response::status::Custom(Status::Ok, Json(response))
    }
}

pub fn AddNoUseConfig(mqtt_config: &MqttConfig, redis_op: &RedisOp) {
    redis_op
        .set_hash(
            "mqtt_config:no",
            mqtt_config.client_id.as_str(),
            serde_json::to_string(mqtt_config).unwrap().as_str(),
        )
        .expect("add no use config 异常");
}
pub fn BindNode(mqtt_config: &MqttConfig, node_name: String, redis_op: &RedisOp) {
    let binding = serde_json::to_string(mqtt_config).unwrap();

    let x = binding.as_str();
    let key = format!("node_bind:{}", node_name);
    redis_op
        .add_set(key.as_str(), mqtt_config.client_id.as_str())
        .unwrap();

    RemoveNoUseConfig(mqtt_config, redis_op);
    AddUseConfig(mqtt_config, redis_op);
}

pub fn AddUseConfig(mqtt_config: &MqttConfig, redis_op: &RedisOp) {
    let binding = serde_json::to_string(mqtt_config).unwrap();
    let x = binding.as_str();
    redis_op
        .set_hash("mqtt_config:use", mqtt_config.client_id.as_str(), x)
        .expect("add use config 异常");
}
pub fn RemoveNoUseConfig(mqtt_config: &MqttConfig, redis_op: &RedisOp) {
    info!("mqtt_config = {:?}", mqtt_config);

    redis_op
        .delete_hash_field("mqtt_config:no", mqtt_config.client_id.as_str())
        .expect("TODO: panic message");
}
pub async fn create_mqtt_client(
    mqtt_config: &MqttConfig,
    redis_op: &RedisOp,
    node_info: &NodeInfo,
) -> i64 {
    let key = format!("node_bind:{}", node_info.name);
    info!("key = {:?}", key);

    let result = redis_op.get_zset_length(key.as_str()).unwrap_or(0) as i64;

    if node_info.size > result {
        let min = create_mqtt_client_min(mqtt_config).await;
        info!("min = {}", min);
        if min {
            return result + 1;
        } else {
            return -2;
        }
    } else {
        return -1;
    }
}

pub async fn create_mqtt_client_min(mqtt_config: &MqttConfig) -> bool {
    match create_client(
        mqtt_config.client_id.as_str(),
        mqtt_config.sub_topic.as_str(),
        mqtt_config.username.as_str(),
        mqtt_config.password.as_str(),
        mqtt_config.broker.as_str(),
        mqtt_config.port as u16,
    )
    .await
    {
        Ok((client1, mut eventloop1)) => {
            let sub_topic = mqtt_config.sub_topic.clone(); // 克隆以获得所有权
            tokio::spawn(event_loop(
                sub_topic,
                eventloop1,
                mqtt_config.client_id.clone(),
            )); // 传递所有权
            true
        }
        Err(_) => {
            // 捕获错误并返回 false
            false
        }
    }
}

pub fn check_has_config(mqtt_client_id: String, redis_op: &RedisOp) -> bool {
    info!("mqtt_client_id = {}", mqtt_client_id);
    return match redis_op.hash_exists("mqtt_config:use", &mqtt_client_id) {
        Ok(e) => return e,
        Err(error) => {
            error!("redis error: {}", error);
            false
        }
    };
}

pub fn GetNoUseConfig(redis_op: &RedisOp) -> Vec<String> {
    return match redis_op.get_hash_all_value("mqtt_config:no") {
        Ok(x) => x,
        Err(error) => {
            error!("redis error: {}", error);
            vec![]
        }
    };
}
#[get("/node_list")]
pub fn NodeList(pool: &rocket::State<RedisOp>) -> &'static str {
    "Counter updated"
}

#[get("/node_using_status")]
pub fn NodeUsingStatus(pool: &rocket::State<RedisOp>) -> &'static str {
    "Counter updated"
}

#[get("/mqtt_config")]
pub fn GetUseMqttConfig(pool: &rocket::State<RedisOp>) -> &'static str {
    "Counter updated"
}

#[get("/no_mqtt_config")]
pub fn GetNoUseMqttConfig(pool: &rocket::State<RedisOp>) -> &'static str {
    "Counter updated"
}

#[get("/remove_mqtt_client")]
pub fn RemoveMqttClient(pool: &rocket::State<RedisOp>) -> &'static str {
    "Counter updated"
}

#[post("/public_create_mqtt")]
pub fn PubCreateMqttClientHttp(pool: &rocket::State<RedisOp>) -> &'static str {
    "Counter updated"
}

#[get("/public_remove_mqtt_client")]
pub fn PubRemoveMqttClient(pool: &rocket::State<RedisOp>) -> &'static str {
    "Counter updated"
}
