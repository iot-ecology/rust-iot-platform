use crate::biz::transmit::mysql_transmit_biz::MysqlTransmitBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/MySQLTransmit/create", format = "json", data = "<data>")]
pub async fn create_mysql_transmit(
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/MySQLTransmit/update", format = "json", data = "<data>")]
pub async fn update_mysql_transmit(
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MySQLTransmit/<id>")]
pub async fn by_id_mysql_transmit(
    id: u64,
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MySQLTransmit/list")]
pub async fn list_mysql_transmit(
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MySQLTransmit/page?<page>&<page_size>")]
pub async fn page_mysql_transmit(
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/MySQLTransmit/delete/<id>")]
pub async fn delete_mysql_transmit(
    id: u64,
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
