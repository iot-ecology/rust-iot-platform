use crate::biz::device_info_biz::DeviceInfoBiz;
use crate::biz::product_biz::ProductBiz;
use crate::db::db_model::DeviceInfo;
use chrono::{Duration, NaiveDate};
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/DeviceInfo/create", format = "json", data = "<data>")]
pub async fn create_device_info(
    data: Json<DeviceInfo>,
    device_info_api: &rocket::State<DeviceInfoBiz>,
    product_api: &rocket::State<ProductBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    if data.sn.clone().is_none() {
        let error_json = json!({
            "code": 40000,
            "message": "操作失败",
            "data": "设备序列号不能为空"
        });
        return Custom(Status::BadRequest, Json(error_json));
    }

    // 检查设备是否已存在
    match device_info_api.find_by_sn(data.sn.clone()).await {
        Ok(existing_device) => {
            if existing_device.is_some() {
                let error_json = json!({
                    "code": 40000,
                    "message": "设备已存在"
                });
                return Custom(Status::BadRequest, Json(error_json));
            }
        }
        Err(e) => {
            let error_json = json!({
                "code": 40000,
                "message": "查询设备失败",
                "data": e.to_string()
            });
            return Custom(Status::InternalServerError, Json(error_json));
        }
    }

    // 获取产品信息
    let product = match product_api.by_id(data.product_id.unwrap()).await {
        Ok(product) => product,
        Err(e) => {
            let error_json = json!({
                "code": 40000,
                "message": "获取产品信息失败",
                "data": e.to_string()
            });
            return Custom(Status::InternalServerError, Json(error_json));
        }
    };

    // 创建设备信息对象
    let mut device_info = data.into_inner();

    // 计算保修期
    if let Some(manufacturing_date) = device_info.manufacturing_date {
        let warranty_expiry = manufacturing_date + Duration::days(product.warranty_period.unwrap() as i64);
        device_info.warranty_expiry = Some(warranty_expiry);
    }

    // 如果是采购设备，使用采购日期作为生产日期
    if device_info.source == Some(2) {
        if let Some(procurement_date) = device_info.procurement_date {
            device_info.manufacturing_date = Some(procurement_date);
            let warranty_expiry = procurement_date + Duration::days(product.warranty_period.unwrap() as i64);
            device_info.warranty_expiry = Some(warranty_expiry);
        }
    }

    // 创建设备
    match device_info_api.create(device_info).await {
        Ok(created_device) => {
            // 更新Redis缓存
            if let Err(e) = device_info_api.set_redis(&created_device).await {
                log::warn!("Failed to update Redis cache: {}", e);
            }

            let success_json = json!({
                "code": 20000,
                "message": "创建成功",
                "data": created_device
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            log::error!("Failed to create device: {}", e);
            let error_json = json!({
                "code": 40000,
                "message": "创建设备失败",
                "data": e.to_string()
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[get("/DeviceInfo/list")]
pub async fn list_device_info(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = device_info_api.list(Vec::new()).await;
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

#[post("/DeviceInfo/update", format = "json", data = "<data>")]
pub async fn update_device_info(
    data: Json<DeviceInfo>,
    device_info_api: &rocket::State<DeviceInfoBiz>,
    product_api: &rocket::State<ProductBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // Get existing device info
    let old_device = match device_info_api.by_id(data.id.unwrap()).await {
        Ok(device) => device,
        Err(e) => {
            error!("Failed to find device: {:?}", e);
            let error_json = json!({
                "code": 40000,
                "message": "设备不存在"
            });
            return Custom(Status::NotFound, Json(error_json));
        }
    };

    let mut new_device = old_device.clone();
    new_device.product_id = data.product_id;
    new_device.source = data.source;
    new_device.manufacturing_date = data.manufacturing_date;
    new_device.procurement_date = data.procurement_date;
    new_device.warranty_expiry = data.warranty_expiry;
    new_device.push_interval = data.push_interval;
    new_device.error_rate = data.error_rate;
    new_device.protocol = data.protocol.clone();

    // Get product info for warranty calculation
    let product = match product_api.by_id(new_device.product_id.unwrap()).await {
        Ok(product) => product,
        Err(e) => {
            error!("Failed to find product: {:?}", e);
            let error_json = json!({
                "code": 40000,
                "message": "产品不存在"
            });
            return Custom(Status::NotFound, Json(error_json));
        }
    };

    // Calculate warranty expiry based on source and manufacturing date
    if new_device.source == Some(2) {
        new_device.manufacturing_date = new_device.procurement_date;
        if let Some(mfg_date) = new_device.manufacturing_date {
            let warranty_days = product.warranty_period.unwrap_or(0) as i64;
            new_device.warranty_expiry = Some(mfg_date + Duration::days(warranty_days));
        }
    } else if let Some(mfg_date) = new_device.manufacturing_date {
        let warranty_days = product.warranty_period.unwrap_or(0) as i64;
        new_device.warranty_expiry = Some(mfg_date + Duration::days(warranty_days));
    }

    // Update device info
    match device_info_api.update(new_device.id.unwrap(), new_device.clone()).await {
        Ok(updated_device) => {
            // Update Redis cache
            if let Err(e) = device_info_api.set_redis(&updated_device).await {
                error!("Failed to update Redis cache: {:?}", e);
            }

            let success_json = json!({
                "code": 20000,
                "message": "更新成功",
                "data": old_device
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            error!("Failed to update device: {:?}", e);
            let error_json = json!({
                "code": 40000,
                "message": "更新失败"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[get("/DeviceInfo/<id>")]
pub async fn by_id_device_info(
    id: u64,
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = device_info_api.delete(id).await;
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

#[get("/DeviceInfo/page?<page>&<page_size>&<name>")]
pub async fn page_device_info(
    page: Option<u64>,
    page_size: Option<u64>,

    name: Option<String>,
    device_info_api: &rocket::State<DeviceInfoBiz>,
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

    let result = device_info_api
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

#[post("/DeviceInfo/delete/<id>")]
pub async fn delete_device_info(
    id: u64,
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = device_info_api.delete(id).await;
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

#[post("/DeviceInfo/BindMqtt")]
pub async fn bind_mqtt(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/DeviceInfo/BindTcp")]
pub async fn bind_tcp(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/DeviceInfo/BindHTTP")]
pub async fn bind_http(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/DeviceInfo/BindHCoap")]
pub async fn bind_hcoap(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/DeviceInfo/BindWebsocket")]
pub async fn bind_websocket(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/DeviceInfo/QueryBindMqtt")]
pub async fn query_bind_mqtt(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/DeviceInfo/QueryBindTcp")]
pub async fn query_bind_tcp(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/DeviceInfo/QueryBindHTTP")]
pub async fn query_bind_http(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/DeviceInfo/QueryBindCoap")]
pub async fn query_bind_coap(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/DeviceInfo/QueryBindWebsocket")]
pub async fn query_bind_websocket(
    device_info_api: &rocket::State<DeviceInfoBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
