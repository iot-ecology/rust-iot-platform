use crate::biz::sim_card_biz::SimCardBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;
#[post("/SimCard/create", format = "json", data = "<data>")]
pub async fn create_sim_card(
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/SimCard/update", format = "json", data = "<data>")]
pub async fn update_sim_card(
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/SimCard/page?<page>&<page_size>")]
pub async fn page_sim_card(
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/SimCard/delete/<id>")]
pub async fn delete_sim_card(
    id: u64,
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/SimCard/<id>")]
pub async fn by_id_sim_card(
    id: u64,
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/SimCard/BindDeviceInfo")]
pub async fn bind_device_info(
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
