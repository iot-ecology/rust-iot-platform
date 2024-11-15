use crate::biz::coap_biz::CoapHandlerBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/CoapHandler/create")]
pub async fn create_coap_handler(
    coap_handler_api: &rocket::State<CoapHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理创建 CoapHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to create CoapHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/CoapHandler/update")]
pub async fn update_coap_handler(
    coap_handler_api: &rocket::State<CoapHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理更新 CoapHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to update CoapHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/CoapHandler/<id>")]
pub async fn by_id_coap_handler(
    id: u64,
    coap_handler_api: &rocket::State<CoapHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理根据 id 获取 CoapHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to find CoapHandler by id"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/CoapHandler/page")]
pub async fn page_coap_handler(
    coap_handler_api: &rocket::State<CoapHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理分页查询 CoapHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to fetch CoapHandler page"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/CoapHandler/delete/<id>")]
pub async fn delete_coap_handler(
    id: u64,
    coap_handler_api: &rocket::State<CoapHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理删除 CoapHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to delete CoapHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}
