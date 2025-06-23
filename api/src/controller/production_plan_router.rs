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
