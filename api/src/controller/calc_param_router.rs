use crate::biz::calc_param_biz::CalcParamBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/calc-param/create", format = "json", data = "<data>")]
pub async fn create_calc_param(
    calc_param_api: &rocket::State<CalcParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-param/update", format = "json", data = "<data>")]
pub async fn update_calc_param(
    calc_param_api: &rocket::State<CalcParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/calc-param/page?<page>&<page_size>")]
pub async fn page_calc_param(
    calc_param_api: &rocket::State<CalcParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-param/delete/<id>")]
pub async fn delete_calc_param(
    id: u64,
    calc_param_api: &rocket::State<CalcParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
