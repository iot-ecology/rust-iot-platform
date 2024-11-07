use crate::mqtt_sample::{create_client, event_loop};
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
use rocket::{get, post, State};
use rocket::{Request, Response};
use serde_json::json;
use std::future::poll_fn;

#[get("/beat")]
pub fn HttpBeat(pool: &rocket::State<RedisOp>) -> &'static str {
    "ok"
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
            rocket::response::status::Custom(Status::BadRequest, Json(response))
        } else {
            let size: i64 = create_mqtt_client(&mqtt_config, &redis_op, &config.node_info);
            let response = json!({
                "status": 400,
                "message": "已经存在客户端id"
            });
            rocket::response::status::Custom(Status::BadRequest, Json(response))
        }
    } else {
        let response = json!({
            "status": 400,
            "message": "已经存在客户端id"
        });
        rocket::response::status::Custom(Status::BadRequest, Json(response))
    }
}

pub fn create_mqtt_client(
    mqtt_config: &MqttConfig,
    redis_op: &RedisOp,
    node_info: &NodeInfo,
) -> i64 {
    let key = format!("node_bind:{}", node_info.name);

    let result = redis_op.get_zset_length(key.as_str()).unwrap_or(0) as i64;

    if node_info.size > result {
        let min = create_mqtt_client_min(mqtt_config);
        info!("min = {}", min);
        return result + 1;
    } else {
        return -1;
    }
}
pub fn create_mqtt_client_min(mqtt_config: &MqttConfig) -> bool {
    let cc = mqtt_config.clone();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (client1, mut eventloop1) = match rt.block_on() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to create MQTT client: {:?}", e);
            return false;
        }
    };
    rt.spawn(async move {
        event_loop(cc.client_id.as_str(), eventloop1).await;
    });

    true
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
