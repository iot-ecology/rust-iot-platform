use crate::biz::shipment_record_biz::ShipmentRecordBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;
#[post("/ShipmentRecord/create")]
pub async fn create_shipment_record(
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ShipmentRecord/update")]
pub async fn update_shipment_record(
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ShipmentRecord/page")]
pub async fn page_shipment_record(
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
