use log::error;
use crate::biz::calc_rule_biz::CalcRuleBiz;

use crate::db::db_model::CalcRule;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;
use crate::biz::calc_run_biz::CalcRunBiz;

#[post("/calc-rule/create", format = "json", data = "<data>")]
pub async fn create_calc_rule(
    data: Json<CalcRule>,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
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

    match calc_rule_api.create(data.into_inner()).await {
        Ok(u) => {
            let success_json = json!({
                            "code": 20000,
                            "message": "创建成功",
                            "data": u
                        });

            calc_run_api.refresh_rule(u.id.unwrap()).await.expect("TODO: panic message");
            calc_run_api.InitMongoCollection(
                &u,
                config.mongo_config.clone().unwrap().clone().collection.unwrap()
            ).await;

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

#[post("/calc-rule/update", format = "json", data = "<data>")]
pub async fn update_calc_rule(
    data: Json<CalcRule>,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    calc_run_api: &rocket::State<CalcRunBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {

    let x = calc_rule_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.name = data.name.clone();
            old.script = data.script.clone();
            old.cron = data.cron.clone();
            old.offset = data.offset.clone();
            old.mock_value = None;
            let result = calc_rule_api.update(old.id.unwrap(), old).await;
            match result {
                Ok(u2) => {
                    let success_json = json!({
                        "code": 20000,
                        "message": "更新成功",
                        "data": u2
                    });
                    calc_run_api.InitMongoCollection(
                        &u2,
                        config.mongo_config.clone().unwrap().clone().collection.unwrap()
                    ).await;

                    calc_run_api.refresh_rule(u2.id.unwrap()).await.expect("TODO: panic message");
                    Custom(rocket::http::Status::Ok, Json(success_json))
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

#[get("/calc-rule/page?<page>&<page_size>&<name>")]
pub async fn page_calc_rule(
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
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
    ];

    let result = calc_rule_api.page(
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

#[post("/calc-rule/delete/<id>")]
pub async fn delete_calc_rule(
    id: u64,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    calc_run_api: &rocket::State<CalcRunBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = calc_rule_api.by_id(id).await;
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

#[post("/calc-rule/start/<id>")]
pub async fn start_calc_rule(
    id: u64,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    calc_run_api: &rocket::State<CalcRunBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-rule/stop/<id>")]
pub async fn stop_calc_rule(
    id: u64,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    calc_run_api: &rocket::State<CalcRunBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/calc-rule/refresh/<id>")]
pub async fn refresh_calc_rule(
    id: u64,
    calc_run_api: &rocket::State<CalcRunBiz>,
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    match calc_run_api.refresh_rule(id).await {
        Ok(_) => {
            let success_json = json!({
                "code": 20000,
                "message": "刷新成功",
            });
            Custom(Status::Ok, Json(success_json))

        }
        Err(_) => {
            let success_json = json!({
                "code": 40000,
                "message": "刷新失败",
            });
            Custom(Status::Ok, Json(success_json))

        }
    }
}

#[post("/calc-rule/mock")]
pub async fn mock_calc_rule(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/calc-rule/rd")]
pub async fn calc_rule_result(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/calc-rule/list")]
pub async fn list_calc_rule(
    calc_rule_api: &rocket::State<CalcRuleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
