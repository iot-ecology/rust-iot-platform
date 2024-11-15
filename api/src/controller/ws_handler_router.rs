use crate::biz::ws_biz::WebSocketHandlerBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/WebsocketHandler/create")]
pub async fn create_websocket_handler(
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理创建 WebsocketHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to create WebsocketHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/WebsocketHandler/update")]
pub async fn update_websocket_handler(
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理更新 WebsocketHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to update WebsocketHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/WebsocketHandler/<id>")]
pub async fn by_id_websocket_handler(
    id: u64,
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理根据 id 获取 WebsocketHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to find WebsocketHandler by id"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/WebsocketHandler/page")]
pub async fn page_websocket_handler(
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理分页查询 WebsocketHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to fetch WebsocketHandler page"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/WebsocketHandler/delete/<id>")]
pub async fn delete_websocket_handler(
    id: u64,
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理删除 WebsocketHandler 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to delete WebsocketHandler"
    });
    Custom(Status::InternalServerError, Json(error_json))
}
