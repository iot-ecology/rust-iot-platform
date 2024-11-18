use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::signal_delay_waring_biz::SignalDelayWaringBiz;
use crate::db::db_model::{SignalDelayWaring, SignalDelayWaringParam};
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal-delay-waring/create", format = "json", data = "<data>")]
pub async fn create_signal_delay_waring(
    data: Json<SignalDelayWaring>,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring/update", format = "json", data = "<data>")]
pub async fn update_signal_delay_waring(
    data: Json<SignalDelayWaring>,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/signal-delay-waring/page?<page>&<page_size>")]
pub async fn page_signal_delay_waring(
    page: Option<u64>,
    page_size: Option<u64>,
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
