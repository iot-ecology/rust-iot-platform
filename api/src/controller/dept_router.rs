use crate::biz::dept_biz::DeptBiz;
use crate::db::db_model::{Dashboard, Dept};
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/Dept/create", format = "json", data = "<data>")]
pub async fn create_dept(
    data: Json<Dept>,
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/Dept/update", format = "json", data = "<data>")]
pub async fn update_dept(
    data: Json<Dept>,
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/Dept/page?<page>&<page_size>")]
pub async fn page_dept(
    page: Option<u64>,
    page_size: Option<u64>,
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/Dept/delete/<id>")]
pub async fn delete_dept(
    id: u64,
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/Dept/<id>")]
pub async fn by_id_dept(
    id: u64,
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/Dept/subs")]
pub async fn find_by_id_subs(
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/Dept/list")]
pub async fn list_dept(
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
