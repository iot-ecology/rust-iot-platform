use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::mqtt_client_biz::MqttClientBiz;
use crate::db::db_model::MqttClient;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/mqtt/create", format = "json", data = "<data>")]
pub async fn create_mqtt(
    data: Json<MqttClient>,
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/mqtt/page?<page>&<page_size>")]
pub async fn page_mqtt(
    page: Option<i64>,
    page_size: Option<i64>,
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/mqtt/list")]
pub async fn list_mqtt(
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/mqtt/byId/<id>")]
pub async fn by_id_mqtt(
    id: i64,
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/mqtt/start")]
pub async fn start_mqtt(
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/mqtt/stop")]
pub async fn stop_mqtt(
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/mqtt/update", format = "json", data = "<data>")]
pub async fn update_mqtt(
    data: Json<MqttClient>,
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/mqtt/delete/<id>")]
pub async fn delete_mqtt(
    id: i64,
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/mqtt/node-using-status")]
pub async fn node_using_status(
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/mqtt/set-script")]
pub async fn set_script(
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/mqtt/check-script")]
pub async fn check_script(
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/mqtt/send")]
pub async fn send_mqtt_message(
    mqtt_api: &rocket::State<MqttClientBiz>,
    config: &rocket::State<Config>,
) -> Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
