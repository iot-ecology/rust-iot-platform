use common_lib::models::MqttConfig;
use common_lib::redis_pool_utils::RedisOp;
use log::info;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use rocket::fairing::AdHoc;
use rocket::http::{Method, Status};
use rocket::serde::json::Json;
use rocket::tokio::time::Duration;
use rocket::{get, post, State};
use rocket::{Request, Response};
use std::future::poll_fn;
#[get("/beat")]
pub fn HttpBeat(pool: &rocket::State<RedisOp>) -> &'static str {
    "ok"
}

#[post("/create_mqtt", format = "json", data = "<mqtt_config>")]
pub fn create_mqtt_client_http(redis_op: &State<RedisOp>, mqtt_config: Json<MqttConfig>) -> Status {
    info!("mqtt_config = {:?}", mqtt_config);

    let option = redis_op.get_string("asfaf").unwrap();
    info!("option = {:?}", option);
    Status::Ok
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
