/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

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
    page: Option<i64>,
    page_size: Option<i64>,
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
    id: i64,
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
    id: i64,
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
    id: i64,
    shipment_record_api: &rocket::State<ShipmentRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
