use crate::biz::signal_delay_waring_param_biz::SignalDelayWaringParamBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal-delay-waring-param/create", format = "json", data = "<data>")]
pub async fn create_signal_delay_waring_param(
    signal_delay_waring_param_api: &rocket::State<SignalDelayWaringParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring-param/update", format = "json", data = "<data>")]
pub async fn update_signal_delay_waring_param(
    signal_delay_waring_param_api: &rocket::State<SignalDelayWaringParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/signal-delay-waring-param/page?<page>&<page_size>")]
pub async fn page_signal_delay_waring_param(
    signal_delay_waring_param_api: &rocket::State<SignalDelayWaringParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring-param/delete/<id>")]
pub async fn delete_signal_delay_waring_param(
    id: u64,
    signal_delay_waring_param_api: &rocket::State<SignalDelayWaringParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
