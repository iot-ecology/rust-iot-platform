use crate::biz::tcp_biz::TcpHandlerBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/TcpHandler/create")]
pub async fn create_tcp_handler(
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

#[post("/TcpHandler/update")]
pub async fn update_tcp_handler(
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

#[get("/TcpHandler/page")]
pub async fn page_tcp_handler(
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
