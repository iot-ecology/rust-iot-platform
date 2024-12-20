use crate::biz::transmit::clickhouse_transmit_biz::ClickhouseTransmitBiz;
use crate::db::db_model::ClickhouseTransmit;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/ClickhouseTransmit/create", format = "json", data = "<data>")]
pub async fn create_clickhouse_transmit(
    data: Json<ClickhouseTransmit>,
    click_transmit_api: &rocket::State<ClickhouseTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ClickhouseTransmit/update", format = "json", data = "<data>")]
pub async fn update_clickhouse_transmit(
    data: Json<ClickhouseTransmit>,
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
    id: i64,
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

#[get("/ClickhouseTransmit/page?<page>&<page_size>")]
pub async fn page_clickhouse_transmit(
    page: Option<i64>,
    page_size: Option<i64>,
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
    id: i64,
    click_transmit_api: &rocket::State<ClickhouseTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
