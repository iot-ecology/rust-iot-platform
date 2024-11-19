use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::production_plan_biz::ProductionPlanBiz;
use crate::db::db_model::ProductionPlan;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/ProductionPlan/create", format = "json", data = "<data>")]
pub async fn create_production_plan(
    data: Json<ProductionPlan>,
    production_plan_api: &rocket::State<ProductionPlanBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ProductionPlan/update", format = "json", data = "<data>")]
pub async fn update_production_plan(
    data: Json<ProductionPlan>,
    production_plan_api: &rocket::State<ProductionPlanBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ProductionPlan/<id>")]
pub async fn by_id_production_plan(
    id: i64,
    production_plan_api: &rocket::State<ProductionPlanBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/ProductionPlan/page?<page>&<page_size>")]
pub async fn page_production_plan(
    page: Option<i64>,
    page_size: Option<i64>,
    production_plan_api: &rocket::State<ProductionPlanBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/ProductionPlan/delete/<id>")]
pub async fn delete_production_plan(
    id: i64,
    production_plan_api: &rocket::State<ProductionPlanBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
