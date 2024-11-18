use crate::biz::notice::feishu_biz::FeiShuBiz;
use crate::db::db_model::{DingDing, FeiShu};
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/FeiShuId/create", format = "json", data = "<data>")]
pub async fn create_feishu(
    data: Json<FeiShu>,
    feishu_api: &rocket::State<FeiShuBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/FeiShuId/update", format = "json", data = "<data>")]
pub async fn update_feishu(
    data: Json<FeiShu>,
    feishu_api: &rocket::State<FeiShuBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/FeiShuId/<id>")]
pub async fn by_id_feishu(
    id: u64,
    feishu_api: &rocket::State<FeiShuBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/FeiShuId/page?<page>&<page_size>")]
pub async fn page_feishu(
    page: Option<u64>,
    page_size: Option<u64>,
    feishu_api: &rocket::State<FeiShuBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/FeiShuId/delete/<id>")]
pub async fn delete_feishu(
    id: u64,
    feishu_api: &rocket::State<FeiShuBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/FeiShuId/bind")]
pub async fn bind_feishu(
    feishu_api: &rocket::State<FeiShuBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
