use crate::biz::calc_param_biz::CalcParamBiz;
use crate::biz::calc_run_biz::CalcRunBiz;
use crate::db::db_model::CalcParam;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/calc-param/create", format = "json", data = "<data>")]
pub async fn create_calc_param(
    data: Json<CalcParam>,
    calc_param_api: &rocket::State<CalcParamBiz>,
    calc_run_api: &rocket::State<CalcRunBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    if data.name.is_none() {
        let error_json = json!({
                            "code": 40000,
                            "message": "名称必填"
                        });
        return Custom(Status::InternalServerError, Json(error_json));
    }

    match calc_param_api.create(data.into_inner()).await {
        Ok(u) => {
            let success_json = json!({
                            "code": 20000,
                            "message": "创建成功",
                            "data": u
                        });

            calc_run_api.refresh_rule(u.id.unwrap()).await;

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

#[post("/calc-param/update", format = "json", data = "<data>")]
pub async fn update_calc_param(
    data: Json<CalcParam>,
    calc_param_api: &rocket::State<CalcParamBiz>,
    calc_run_api: &rocket::State<CalcRunBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = calc_param_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.name = data.name.clone();
            old.reduce = data.reduce.clone();
            old.protocol = data.protocol.clone();
            old.signal_id = data.signal_id.clone();
            old.device_uid = data.device_uid.clone();
            old.identification_code = data.identification_code.clone();
            let result = calc_param_api.update(old.id.unwrap(), old).await;
            match result {
                Ok(u2) => {
                    let success_json = json!({
                        "code": 20000,
                        "message": "更新成功",
                        "data": u2
                    });
                    calc_run_api.refresh_rule(u2.id.unwrap()).await;
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

#[get("/calc-param/page?<page>&<page_size>&<name>&<rule_id>&<signal_name>")]
pub async fn page_calc_param(
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
    rule_id: Option<u64>,
    signal_name: Option<u64>,
    calc_param_api: &rocket::State<CalcParamBiz>,
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

    let filters = vec![
        FilterInfo {
            field: "name".to_string(),
            value: name.unwrap_or_default(),
            operation: FilterOperation::AllLike,
            value2: None,
        },
        FilterInfo {
            field: "calc_rule_id".to_string(),
            value: rule_id.unwrap_or_default().to_string(),
            operation: FilterOperation::Equal,
            value2: None,
        },
        FilterInfo {
            field: "signal_id".to_string(),
            value: signal_name.unwrap_or_default().to_string(),
            operation: FilterOperation::Equal,
            value2: None,
        },
    ];

    let result = calc_param_api.page(
        filters,
        PaginationParams {
            page: page,
            size: page_size,
        },
    ).await;

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

#[post("/calc-param/delete/<id>")]
pub async fn delete_calc_param(
    id: u64,
    calc_param_api: &rocket::State<CalcParamBiz>,
    calc_run_api: &rocket::State<CalcRunBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = calc_param_api.by_id(id).await;
    match result {
        Ok(u) => {
            let success_json = json!({
                        "code": 20000,
                        "message": "查询成功",
            "data":u
                    });
            calc_run_api.refresh_rule(u.id.unwrap()).await.expect("TODO: panic message");
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
