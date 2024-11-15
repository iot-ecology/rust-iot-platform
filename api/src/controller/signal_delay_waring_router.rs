use crate::biz::signal_delay_waring_biz::SignalDelayWaringBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal-delay-waring/create")]
pub async fn create_signal_delay_waring(
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring/update")]
pub async fn update_signal_delay_waring(
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/signal-delay-waring/page")]
pub async fn page_signal_delay_waring(
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring/delete/<id>")]
pub async fn delete_signal_delay_waring(
    id: u64,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring/Mock/<id>")]
pub async fn mock_signal_delay_waring(
    id: u64,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring/GenParam/<id>")]
pub async fn gen_param_signal_delay_waring(
    id: u64,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring/query-row")]
pub async fn query_waring_list(
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/signal-delay-waring/list")]
pub async fn list_signal_delay_waring(
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/signal-delay-waring/byId/<id>")]
pub async fn by_id_signal_delay_waring(
    id: u64,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
