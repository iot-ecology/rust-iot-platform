use crate::biz::transmit::cassandra_transmit_biz::CassandraTransmitBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/CassandraTransmit/create")]
pub async fn create_cassandra_transmit(
    cassandra_transmit_api: &rocket::State<CassandraTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/CassandraTransmit/update")]
pub async fn update_cassandra_transmit(
    cassandra_transmit_api: &rocket::State<CassandraTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/CassandraTransmit/<id>")]
pub async fn by_id_cassandra_transmit(
    id: u64,
    cassandra_transmit_api: &rocket::State<CassandraTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/CassandraTransmit/list")]
pub async fn list_cassandra_transmit(
    cassandra_transmit_api: &rocket::State<CassandraTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/CassandraTransmit/page")]
pub async fn page_cassandra_transmit(
    cassandra_transmit_api: &rocket::State<CassandraTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/CassandraTransmit/delete/<id>")]
pub async fn delete_cassandra_transmit(
    id: u64,
    cassandra_transmit_api: &rocket::State<CassandraTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
