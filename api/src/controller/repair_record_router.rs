use log::error;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::repair_record_biz::RepairRecordBiz;
use crate::db::db_model::{ProductionPlan, RepairRecord};
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/RepairRecord/create", format = "json", data = "<data>")]
pub async fn create_repair_record(
    data: Json<RepairRecord>,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {

    match repair_record_api.create(data.into_inner()).await {
        Ok(u) => {
            let success_json = json!({
                            "code": 20000,
                            "message": "创建成功",
                            "data": u
                        });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let error_json = json!({
                            "code": 40000,
                            "message": "创建失败"
                        });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[post("/RepairRecord/update", format = "json", data = "<data>")]
pub async fn update_repair_record(
    data: Json<RepairRecord>,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = repair_record_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.device_group_group_id = data.device_group_group_id.clone();
            old.device_info_id = data.device_info_id.clone();
            old.repair_date = data.repair_date.clone();
            old.technician = data.technician.clone();
            old.cost = data.cost.clone();
            old.description = data.description.clone();
            let result = repair_record_api.update(old.id.unwrap(), old).await;
            match result {
                Ok(u2) => {
                    let success_json = json!({
                        "code": 20000,
                        "message": "更新成功",
                        "data": u2
                    });
                    Custom(Status::Ok, Json(success_json))
                }
                Err(e) => {
                    error!("error =  {:?}", e);

                    let error_json = json!({
                        "code": 40000,
                        "message": "查询失败"
                    });
                    Custom(Status::InternalServerError, Json(error_json))
                }
            }
        }
        Err(e) => {
            error!("error =  {:?}", e);

            let error_json = json!({
                "code": 40000,
                "message": "查询失败"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[get("/RepairRecord/<id>")]
pub async fn by_id_repair_record(
    id: u64,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = repair_record_api.by_id(id).await;
    match result {
        Ok(u) => {
            let success_json = json!({
                        "code": 20000,
                        "message": "查询成功",
            "data":u
                    });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let success_json = json!({
                "code": 40000,
                "message": "查询失败",
            });
            Custom(Status::Ok, Json(success_json))
        }
    }
}

#[get("/RepairRecord/page?<page>&<page_size>&<name>")]
pub async fn page_repair_record(
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);

    if page == 0 || page_size == 0 {
        let error_json = json!({
                           "code": 40000,

            "message": "Invalid page or page_size parameters"
        });
        return Custom(Status::Ok, Json(error_json));
    };

    let filters = vec![FilterInfo {
        field: "name".to_string(),
        value: name.unwrap_or_else(String::new),
        operation: FilterOperation::AllLike,
        value2: None,
    }];

    let result = repair_record_api
        .page(
            filters,
            PaginationParams {
                page: page,
                size: page_size,
            },
        )
        .await;

    match result {
        Ok(uu) => {
            let success_json = json!({
                "code": 20000,
                "message": "查询成功",
                "data": uu
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let error_json = json!({
                               "code": 40000,

                    "message": "查询失败"

            });
            return Custom(Status::Ok, Json(error_json));
        }
    }
}

#[post("/RepairRecord/delete/<id>")]
pub async fn delete_repair_record(
    id: u64,
    repair_record_api: &rocket::State<RepairRecordBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = repair_record_api.delete(id).await;
    match result {
        Ok(o) => {
            let success_json = json!({
                "code": 20000,
                "message": "删除成功",
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let success_json = json!({
                "code": 40000,
                "message": "删除失败",
            });
            Custom(Status::Ok, Json(success_json))
        }
    }
}
