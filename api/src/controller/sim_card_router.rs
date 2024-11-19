use crate::biz::sim_card_biz::SimCardBiz;
use crate::db::db_model::SimCard;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/SimCard/create", format = "json", data = "<data>")]
pub async fn create_sim_card(
    data: Json<SimCard>,
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    match sim_card_api.create(data.into_inner()).await {
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

#[post("/SimCard/update", format = "json", data = "<data>")]
pub async fn update_sim_card(
    data: Json<SimCard>,
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = sim_card_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.access_number = data.access_number.clone();
            old.iccid = data.iccid.clone();
            old.imsi = data.imsi.clone();
            old.operator = data.operator.clone();
            old.expiration = data.expiration.clone();
            let result = sim_card_api.update(old.id.unwrap(), old).await;
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

#[get("/SimCard/page?<page>&<page_size>&<AccessNumber>&<iccid>")]
pub async fn page_sim_card(
    page: Option<i64>,
    page_size: Option<i64>,
    AccessNumber: Option<String>,
    iccid: Option<String>,
    sim_card_api: &rocket::State<SimCardBiz>,
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
            field: "access_number".to_string(),
            value: AccessNumber.unwrap_or_else(String::new),
            operation: FilterOperation::AllLike,
            value2: None,
        },
        FilterInfo {
            field: "iccid".to_string(),
            value: iccid.unwrap_or_else(String::new),
            operation: FilterOperation::AllLike,
            value2: None,
        },
    ];

    let result = sim_card_api
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

#[post("/SimCard/delete/<id>")]
pub async fn delete_sim_card(
    id: i64,
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = sim_card_api.delete(id).await;
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

#[get("/SimCard/<id>")]
pub async fn by_id_sim_card(
    id: i64,
    sim_card_api: &rocket::State<SimCardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = sim_card_api.by_id(id).await;
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
