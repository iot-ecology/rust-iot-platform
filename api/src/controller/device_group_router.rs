use crate::biz::device_group_biz::DeviceGroupBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;
#[post("/device_group/create", format = "json", data = "<data>")]
pub async fn create_device_group(
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/device_group/update", format = "json", data = "<data>")]
pub async fn update_device_group(
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/device_group/<id>")]
pub async fn by_id_device_group(
    id: u64,
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/device_group/page?<page>&<page_size>")]
pub async fn page_device_group(
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/device_group/delete/<id>")]
pub async fn delete_device_group(
    id: u64,
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/device_group/query_bind_device")]
pub async fn query_bind_device_info(
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/device_group/bind_device")]
pub async fn bind_device_info(
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/device_group/BindMqtt")]
pub async fn bind_mqtt(
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/device_group/QueryBindMqtt")]
pub async fn query_bind_mqtt(
    device_group_api: &rocket::State<DeviceGroupBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
