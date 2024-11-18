use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::repair_record_biz::RepairRecordBiz;
use crate::db::db_model::{ProductionPlan, RepairRecord};
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/RepairRecord/create", format = "json", data = "<data>")]
pub async fn create_repair_record(
    data: Json<RepairRecord>,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/RepairRecord/update", format = "json", data = "<data>")]
pub async fn update_repair_record(
    data: Json<RepairRecord>,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/RepairRecord/<id>")]
pub async fn by_id_repair_record(
    id: u64,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/RepairRecord/page?<page>&<page_size>")]
pub async fn page_repair_record(
    page: Option<u64>,
    page_size: Option<u64>,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/RepairRecord/delete/<id>")]
pub async fn delete_repair_record(
    id: u64,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
