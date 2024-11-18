use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::signal_delay_waring_biz::SignalDelayWaringBiz;
use crate::db::db_model::{Signal, SignalDelayWaring};
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal-waring-config/create", format = "json", data = "<data>")]
pub async fn create_signal_waring_config(
    data: Json<SignalDelayWaring>,
    signal_waring_config_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-waring-config/delete/<id>")]
pub async fn delete_signal_waring_config(
    id: u64,
    signal_waring_config_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-waring-config/query-row")]
pub async fn query_waring_list(
    signal_waring_config_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-waring-config/update", format = "json", data = "<data>")]
pub async fn update_signal_waring_config(
    data: Json<SignalDelayWaring>,
    signal_waring_config_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/signal-waring-config/page?<page>&<page_size>")]
pub async fn page_signal_waring_config(
    page: Option<u64>,
    page_size: Option<u64>,
    signal_waring_config_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
