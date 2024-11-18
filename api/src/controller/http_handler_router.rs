use crate::biz::http_biz::HttpHandlerBiz;

use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/HttpHandler/create", format = "json", data = "<data>")]
pub async fn create_http_handler(
    http_handler_api: &rocket::State<HttpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理创建 HttpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to create HttpHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/HttpHandler/update", format = "json", data = "<data>")]
pub async fn update_http_handler(
    http_handler_api: &rocket::State<HttpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理更新 HttpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to update HttpHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/HttpHandler/<id>")]
pub async fn by_id_http_handler(
    id: u64,
    http_handler_api: &rocket::State<HttpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理根据 id 获取 HttpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to find HttpHandler by id"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/HttpHandler/page?<page>&<page_size>")]
pub async fn page_http_handler(
    http_handler_api: &rocket::State<HttpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理分页查询 HttpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to fetch HttpHandler page"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/HttpHandler/delete/<id>")]
pub async fn delete_http_handler(
    id: u64,
    http_handler_api: &rocket::State<HttpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理删除 HttpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to delete HttpHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}
