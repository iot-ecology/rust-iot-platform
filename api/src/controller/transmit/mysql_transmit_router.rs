/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::biz::transmit::mysql_transmit_biz::MysqlTransmitBiz;
use crate::db::db_model::{MongoTransmit, MysqlTransmit};
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/MySQLTransmit/create", format = "json", data = "<data>")]
pub async fn create_mysql_transmit(
    data: Json<MysqlTransmit>,
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/MySQLTransmit/update", format = "json", data = "<data>")]
pub async fn update_mysql_transmit(
    data: Json<MysqlTransmit>,
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MySQLTransmit/<id>")]
pub async fn by_id_mysql_transmit(
    id: i64,
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MySQLTransmit/list")]
pub async fn list_mysql_transmit(
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/MySQLTransmit/page?<page>&<page_size>")]
pub async fn page_mysql_transmit(
    page: Option<i64>,
    page_size: Option<i64>,
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/MySQLTransmit/delete/<id>")]
pub async fn delete_mysql_transmit(
    id: i64,
    mysql_transmit_api: &rocket::State<MysqlTransmitBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
