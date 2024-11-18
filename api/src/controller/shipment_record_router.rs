use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::shipment_record_biz::ShipmentRecordBiz;
use crate::db::db_model::ShipmentRecord;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/ShipmentRecord/create", format = "json", data = "<data>")]
pub async fn create_shipment_record(
    data: Json<ShipmentRecord>,
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ShipmentRecord/update", format = "json", data = "<data>")]
pub async fn update_shipment_record(
    data: Json<ShipmentRecord>,
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ShipmentRecord/page?<page>&<page_size>")]
pub async fn page_shipment_record(
    page: Option<u64>,
    page_size: Option<u64>,
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ShipmentRecord/delete/<id>")]
pub async fn delete_shipment_record(
    id: u64,
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ShipmentRecord/<id>")]
pub async fn by_id_shipment_record(
    id: u64,
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ShipmentRecord/FindByShipmentProductDetail/<id>")]
pub async fn find_by_shipment_product_detail(
    id: u64,
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
