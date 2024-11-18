use crate::biz::dashboard_biz::DashboardBiz;
use crate::biz::mqtt_client_biz::MqttClientBiz;
use crate::db::db_model::Dashboard;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/dashboard/create", format = "json", data = "<data>")]
pub async fn create_dashboard(
    data: Json<Dashboard>,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/dashboard/update", format = "json", data = "<data>")]
pub async fn update_dashboard(
    data: Json<Dashboard>,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/dashboard/<id>")]
pub async fn by_id_dashboard(
    id: u64,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/dashboard/page?<page>&<page_size>")]
pub async fn page_dashboard(
    page: Option<u64>,
    page_size: Option<u64>,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/dashboard/delete/<id>")]
pub async fn delete_dashboard(
    id: u64,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
