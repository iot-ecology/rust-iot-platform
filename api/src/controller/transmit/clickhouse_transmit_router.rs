use crate::biz::transmit::clickhouse_transmit_biz::ClickhouseTransmitBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/ClickhouseTransmit/create")]
pub async fn create_clickhouse_transmit(
    click_transmit_api: &rocket::State<ClickhouseTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ClickhouseTransmit/update")]
pub async fn update_clickhouse_transmit(
    click_transmit_api: &rocket::State<ClickhouseTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ClickhouseTransmit/<id>")]
pub async fn by_id_clickhouse_transmit(
    id: u64,
    click_transmit_api: &rocket::State<ClickhouseTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ClickhouseTransmit/list")]
pub async fn list_clickhouse_transmit(
    click_transmit_api: &rocket::State<ClickhouseTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ClickhouseTransmit/page")]
pub async fn page_clickhouse_transmit(
    click_transmit_api: &rocket::State<ClickhouseTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ClickhouseTransmit/delete/<id>")]
pub async fn delete_clickhouse_transmit(
    id: u64,
    click_transmit_api: &rocket::State<ClickhouseTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
