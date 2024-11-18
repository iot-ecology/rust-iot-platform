use crate::biz::transmit::mongo_transmit_biz::MongoTransmitBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/MongoTransmit/create", format = "json", data = "<data>")]
pub async fn create_mongo_transmit(
    mongo_transmit_api: &rocket::State<MongoTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/MongoTransmit/update", format = "json", data = "<data>")]
pub async fn update_mongo_transmit(
    mongo_transmit_api: &rocket::State<MongoTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MongoTransmit/<id>")]
pub async fn by_id_mongo_transmit(
    id: u64,
    mongo_transmit_api: &rocket::State<MongoTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MongoTransmit/list")]
pub async fn list_mongo_transmit(
    mongo_transmit_api: &rocket::State<MongoTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MongoTransmit/page?<page>&<page_size>")]
pub async fn page_mongo_transmit(
    mongo_transmit_api: &rocket::State<MongoTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/MongoTransmit/delete/<id>")]
pub async fn delete_mongo_transmit(
    id: u64,
    mongo_transmit_api: &rocket::State<MongoTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
