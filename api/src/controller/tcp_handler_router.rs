use crate::biz::tcp_biz::TcpHandlerBiz;
use crate::db::db_model::{SimCard, TcpHandler};
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/TcpHandler/create", format = "json", data = "<data>")]
pub async fn create_tcp_handler(
    data: Json<TcpHandler>,
    tcp_handler_api: &rocket::State<TcpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理创建 TcpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to create TcpHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/TcpHandler/update", format = "json", data = "<data>")]
pub async fn update_tcp_handler(
    data: Json<TcpHandler>,
    tcp_handler_api: &rocket::State<TcpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理更新 TcpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to update TcpHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/TcpHandler/<id>")]
pub async fn by_id_tcp_handler(
    id: u64,
    tcp_handler_api: &rocket::State<TcpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理根据 id 获取 TcpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to find TcpHandler by id"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/TcpHandler/page?<page>&<page_size>")]
pub async fn page_tcp_handler(
    page: Option<u64>,
    page_size: Option<u64>,
    tcp_handler_api: &rocket::State<TcpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理分页查询 TcpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to fetch TcpHandler page"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/TcpHandler/delete/<id>")]
pub async fn delete_tcp_handler(
    id: u64,
    tcp_handler_api: &rocket::State<TcpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理删除 TcpHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to delete TcpHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}
