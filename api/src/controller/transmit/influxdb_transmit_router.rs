use crate::biz::transmit::influxdb_transmit_biz::InfluxDbTransmitBiz;
use crate::db::db_model::InfluxDbTransmit;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/InfluxdbTransmit/create", format = "json", data = "<data>")]
pub async fn create_influxdb_transmit(
    data: Json<InfluxDbTransmit>,
    influxdb_transmit_api: &rocket::State<InfluxDbTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/InfluxdbTransmit/update", format = "json", data = "<data>")]
pub async fn update_influxdb_transmit(
    data: Json<InfluxDbTransmit>,
    influxdb_transmit_api: &rocket::State<InfluxDbTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/InfluxdbTransmit/<id>")]
pub async fn by_id_influxdb_transmit(
    id: i64,
    influxdb_transmit_api: &rocket::State<InfluxDbTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/InfluxdbTransmit/list")]
pub async fn list_influxdb_transmit(
    influxdb_transmit_api: &rocket::State<InfluxDbTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/InfluxdbTransmit/page?<page>&<page_size>")]
pub async fn page_influxdb_transmit(
    page: Option<i64>,
    page_size: Option<i64>,
    influxdb_transmit_api: &rocket::State<InfluxDbTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/InfluxdbTransmit/delete/<id>")]
pub async fn delete_influxdb_transmit(
    id: i64,
    influxdb_transmit_api: &rocket::State<InfluxDbTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
