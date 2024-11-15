use crate::biz::signal_delay_waring_biz::SignalDelayWaringBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal-waring-config/create")]
pub async fn create_signal_waring_config(
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

#[post("/signal-waring-config/update")]
pub async fn update_signal_waring_config(
    signal_waring_config_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/signal-waring-config/page")]
pub async fn page_signal_waring_config(
    signal_waring_config_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
