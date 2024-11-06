use common_lib::models::MqttConfig;
use common_lib::rabbit_utils::RedisPool;
use rocket::fairing::AdHoc;
use rocket::http::{Method, Status};
use rocket::serde::json::Json;
use rocket::tokio::time::Duration;
use rocket::{get, post, State};
use rocket::{Request, Response};

#[get("/beat")]
pub fn HttpBeat(pool: &rocket::State<RedisPool>) -> &'static str {
    "ok"
}

#[post("/create_mqtt", format = "json", data = "<mqtt_config>")]
pub fn create_mqtt_client_http(pool: &State<RedisPool>, mqtt_config: Json<MqttConfig>) -> Status {
    // 打印接收到的 MqttConfig 数据
    println!("{:?}", mqtt_config);

    // 你可以在这里使用 pool 和 mqtt_config 进一步处理逻辑

    Status::Ok
}

#[get("/node_list")]
pub fn NodeList(pool: &rocket::State<RedisPool>) -> &'static str {
    "Counter updated"
}

#[get("/node_using_status")]
pub fn NodeUsingStatus(pool: &rocket::State<RedisPool>) -> &'static str {
    "Counter updated"
}

#[get("/mqtt_config")]
pub fn GetUseMqttConfig(pool: &rocket::State<RedisPool>) -> &'static str {
    "Counter updated"
}

#[get("/no_mqtt_config")]
pub fn GetNoUseMqttConfig(pool: &rocket::State<RedisPool>) -> &'static str {
    "Counter updated"
}

#[get("/remove_mqtt_client")]
pub fn RemoveMqttClient(pool: &rocket::State<RedisPool>) -> &'static str {
    "Counter updated"
}

#[post("/public_create_mqtt")]
pub fn PubCreateMqttClientHttp(pool: &rocket::State<RedisPool>) -> &'static str {
    "Counter updated"
}

#[get("/public_remove_mqtt_client")]
pub fn PubRemoveMqttClient(pool: &rocket::State<RedisPool>) -> &'static str {
    "Counter updated"
}
