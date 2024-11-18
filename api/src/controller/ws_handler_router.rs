use crate::biz::ws_biz::WebSocketHandlerBiz;
use crate::db::db_model::WebSocketHandler;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/WebsocketHandler/create", format = "json", data = "<data>")]
pub async fn create_websocket_handler(
    data: Json<WebSocketHandler>,
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    if data.name.clone().is_none() {
        let error_json = json!({
            "code": 40000,
            "message": "操作失败",
            "data": "名称不能为空"
        });
        return Custom(Status::InternalServerError, Json(error_json));
    }

    match ws_handler_api.create(data.into_inner()).await {
        Ok(u) => {
            let success_json = json!({
                "code": 20000,
                "message": "创建成功",
                "data": u
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let error_json = json!({
                "code": 40000,
                "message": "创建失败"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[post("/WebsocketHandler/update", format = "json", data = "<data>")]
pub async fn update_websocket_handler(
    data: Json<WebSocketHandler>,
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

#[get("/WebsocketHandler/page?<page>&<page_size>")]
pub async fn page_websocket_handler(
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
    page: Option<u64>,
    page_size: Option<u64>,
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
