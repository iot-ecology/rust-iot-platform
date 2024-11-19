use crate::biz::notice::dingding_biz::DingDingBiz;
use crate::db::db_model::DingDing;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/DingDing/create", format = "json", data = "<data>")]
pub async fn create_dingding(
    data: Json<DingDing>,
    dingding_api: &rocket::State<DingDingBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/DingDing/update", format = "json", data = "<data>")]
pub async fn update_dingding(
    data: Json<DingDing>,
    dingding_api: &rocket::State<DingDingBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/DingDing/<id>")]
pub async fn by_id_dingding(
    id: i64,
    dingding_api: &rocket::State<DingDingBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/DingDing/page?<page>&<page_size>")]
pub async fn page_dingding(
    page: Option<i64>,
    page_size: Option<i64>,
    dingding_api: &rocket::State<DingDingBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/DingDing/delete/<id>")]
pub async fn delete_dingding(
    id: i64,
    dingding_api: &rocket::State<DingDingBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/DingDing/bind")]
pub async fn bind_dingding(
    dingding_api: &rocket::State<DingDingBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
