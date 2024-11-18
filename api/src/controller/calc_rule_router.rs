use crate::biz::calc_rule_biz::CalcRuleBiz;

use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/calc-rule/create", format = "json", data = "<data>")]
pub async fn create_calc_rule(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-rule/update", format = "json", data = "<data>")]
pub async fn update_calc_rule(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/calc-rule/page?<page>&<page_size>")]
pub async fn page_calc_rule(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-rule/delete/<id>")]
pub async fn delete_calc_rule(
    id: u64,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-rule/start/<id>")]
pub async fn start_calc_rule(
    id: u64,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-rule/stop/<id>")]
pub async fn stop_calc_rule(
    id: u64,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-rule/refresh/<id>")]
pub async fn refresh_calc_rule(
    id: u64,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-rule/mock")]
pub async fn mock_calc_rule(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/calc-rule/rd")]
pub async fn calc_rule_result(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/calc-rule/list")]
pub async fn list_calc_rule(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
