use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;

use crate::biz::signal_biz::SignalBiz;
use crate::db::db_model::{Signal, SignalDelayWaring};
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal/create", format = "json", data = "<data>")]
pub async fn create_signal(
    data: Json<Signal>,
    signal_api: &rocket::State<SignalBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // Validate signal name
    if data.name.as_ref().map_or(true, |name| name.is_empty()) {
        let error_json = json!({
            "code": 40000,
            "message": "信号名称不能为空"
        });
        return Custom(Status::BadRequest, Json(error_json));
    }

    // Create signal in database
    match signal_api.create(data.into_inner()).await {
        Ok(created_signal) => {
            // Set signal cache
            if let Err(e) = signal_api.set_signal_cache(&created_signal).await {
                log::warn!("Failed to set signal cache: {}", e);
            }

            let success_json = json!({
                "code": 20000,
                "message": "创建成功",
                "data": created_signal
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            log::error!("Failed to create signal: {}", e);
            let error_json = json!({
                "code": 40000,
                "message": "创建失败",
                "data": e.to_string()
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[post("/signal/update", format = "json", data = "<data>")]
pub async fn update_signal(
    data: Json<Signal>,
    signal_api: &rocket::State<SignalBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = signal_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.protocol = data.protocol.clone();
            old.identification_code = data.identification_code.clone();
            old.device_uid = data.device_uid.clone();
            old.name = data.name.clone();
            old.alias = data.alias.clone();
            old.signal_type = data.signal_type.clone();
            old.unit = data.unit.clone();
            old.cache_size = data.cache_size.clone();
            signal_api.remove_signal_cache(&old).await.unwrap();
            let result = signal_api.update(old.id.unwrap(), old).await;
            match result {
                Ok(u2) => {
                    let success_json = json!({
                        "code": 20000,
                        "message": "更新成功",
                        "data": u2
                    });


                    let _ = signal_api.remove_old_cache(u2.cache_size.unwrap(),
                                                        u2.id.unwrap(),
                                                        u2.device_uid.unwrap(),
                                                        u2.identification_code.clone().unwrap().as_str(),
                    ).await;

                    if let Err(e) = signal_api.set_signal_cache(&u2).await {
                        log::warn!("Failed to set signal cache: {}", e);
                    }
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

#[post("/signal/delete/<id>")]
pub async fn delete_signal(
    id: u64,
    signal_api: &rocket::State<SignalBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = signal_api.delete(id).await;
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

#[get("/signal/page?<page>&<page_size>&<device_uid>&<protocol>&<type>")]
pub async fn page_signal(
    page: Option<u64>,
    page_size: Option<u64>,
    device_uid: Option<String>,
    protocol: Option<String>,
    r#type: Option<String>,
    signal_api: &rocket::State<SignalBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let page = page.unwrap_or(0);
    let page_size = page_size.unwrap_or(10);
    let protocol = protocol.unwrap_or_else(|| "MQTT".to_string());
    let ty = r#type.unwrap_or_else(|| "数字".to_string());

    if page_size == 0 {
        let error_json = json!({
            "code": 40000,
            "message": "无效的页长"
        });
        return Custom(Status::BadRequest, Json(error_json));
    }

    let filters = vec![
        FilterInfo {
            field: "device_uid".to_string(),
            value: device_uid.unwrap_or_default(),
            operation: FilterOperation::Equal,
            value2: None,
        },
        FilterInfo {
            field: "protocol".to_string(),
            value: protocol,
            operation: FilterOperation::Equal,
            value2: None,
        },
        FilterInfo {
            field: "type".to_string(),
            value: ty,
            operation: FilterOperation::Equal,
            value2: None,
        },
    ];

    match signal_api
        .page(
            filters,
            PaginationParams {
                page,
                size: page_size,
            },
        )
        .await
    {
        Ok(data) => {
            let success_json = json!({
                "code": 20000,
                "message": "查询成功",
                "data": data
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(_) => {
            let error_json = json!({
                "code": 40000,
                "message": "查询异常"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[get("/signal/byId/<id>")]
pub async fn signal_by_id(
    id: u64,
    signal_api: &rocket::State<SignalBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = signal_api.by_id(id).await;
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

#[get("/signal/initCache")]
pub async fn init_cache(
    signal_api: &rocket::State<SignalBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 获取所有信号
    let result = signal_api.list(Vec::new()).await;
    match result {
        Ok(signals) => {
            // 遍历所有信号并设置缓存
            for signal in signals {
                if let Err(e) = signal_api.set_signal_cache(&signal).await {
                    log::error!("Failed to set signal cache: {}", e);
                }
            }

            let success_json = json!({
                "code": 20000,
                "message": "缓存初始化成功",
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            log::error!("Failed to get signals for cache initialization: {}", e);
            let error_json = json!({
                "code": 40000,
                "message": "缓存初始化失败",
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[get("/signal/list")]
pub async fn list_signal(
    signal_api: &rocket::State<SignalBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = signal_api.list(Vec::new()).await;
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
