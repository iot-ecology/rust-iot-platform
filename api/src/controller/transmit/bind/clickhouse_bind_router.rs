use crate::biz::transmit::bind::clickhouse_bind_biz::ClickhouseTransmitBindBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/ClickhouseTransmitBind/create", format = "json", data = "<data>")]
pub async fn create_clickhouse_transmit_bind(
    clickhouse_transmit_bind_api: &rocket::State<ClickhouseTransmitBindBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理创建 ClickhouseTransmitBind 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to create ClickhouseTransmitBind"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ClickhouseTransmitBind/update", format = "json", data = "<data>")]
pub async fn update_clickhouse_transmit_bind(
    clickhouse_transmit_bind_api: &rocket::State<ClickhouseTransmitBindBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理更新 ClickhouseTransmitBind 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to update ClickhouseTransmitBind"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ClickhouseTransmitBind/<id>")]
pub async fn by_id_clickhouse_transmit_bind(
    id: u64,
    clickhouse_transmit_bind_api: &rocket::State<ClickhouseTransmitBindBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理根据 id 获取 ClickhouseTransmitBind 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to find ClickhouseTransmitBind by id"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ClickhouseTransmitBind/page?<page>&<page_size>")]
pub async fn page_clickhouse_transmit_bind(
    clickhouse_transmit_bind_api: &rocket::State<ClickhouseTransmitBindBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理分页查询 ClickhouseTransmitBind 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to fetch ClickhouseTransmitBind page"
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ClickhouseTransmitBind/delete/<id>")]
pub async fn delete_clickhouse_transmit_bind(
    id: u64,
    clickhouse_transmit_bind_api: &rocket::State<ClickhouseTransmitBindBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 处理删除 ClickhouseTransmitBind 的逻辑
    let error_json = json!({
        "status": "error",
        "message": "Failed to delete ClickhouseTransmitBind"
    });
    Custom(Status::InternalServerError, Json(error_json))
}
