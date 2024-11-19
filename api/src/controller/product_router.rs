use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;

use crate::biz::product_biz::ProductBiz;
use crate::db::db_model::{MqttClient, Product};
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/product/create", format = "json", data = "<data>")]
pub async fn create_product(
    data: Json<Product>,
    product_api: &rocket::State<ProductBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    if data.name.clone().is_none() {
        let error_json = json!({
            "code": 40000,
            "message": "操作失败",
            "data": "名称不能为空"
        });
        return Custom(Status::InternalServerError, Json(error_json));
    }

    match product_api.find_by_name(data.name.clone()).await {
        Ok(u) => {
            if u.is_none() {
                match product_api.create(data.into_inner()).await {
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
            } else {
                let error_json = json!({
                    "code": 40000,
                    "message": "角色已存在"
                });
                Custom(Status::InternalServerError, Json(error_json))
            }
        }
        Err(e) => {
            let error_json = json!({
                "code": 40000,
                "message": "查询失败"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[post("/product/update", format = "json", data = "<data>")]
pub async fn update_product(
    data: Json<Product>,
    product_api: &rocket::State<ProductBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = product_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.description = data.description.clone();
            old.sku = data.sku.clone();
            old.price = data.price.clone();
            old.cost = data.cost.clone();
            old.quantity = data.quantity.clone();
            old.minimum_stock = data.minimum_stock.clone();
            old.warranty_period = data.warranty_period.clone();
            old.status = data.status.clone();
            old.tags = data.tags.clone();
            old.image_url = data.image_url.clone();
            let result = product_api.update(old.id.unwrap(), old).await;
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

#[get("/product/<id>")]
pub async fn by_id_product(
    id: i64,
    product_api: &rocket::State<ProductBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = product_api.by_id(id).await;
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
            log::error!("e ={}",e);

            let success_json = json!({
                "code": 40000,
                "message": "查询失败",
            });
            Custom(Status::Ok, Json(success_json))
        }
    }
}

#[get("/product/list")]
pub async fn list_product(
    product_api: &rocket::State<ProductBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = product_api.list(Vec::new()).await;
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
            log::error!("e ={}",e);
            let success_json = json!({
                "code": 40000,
                "message": "查询失败",
            });
            Custom(Status::Ok, Json(success_json))
        }
    }
}

#[get("/product/page?<page>&<page_size>&<name>")]
pub async fn page_product(
    page: Option<i64>,
    page_size: Option<i64>,
    name: Option<String>,
    product_api: &rocket::State<ProductBiz>,
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

    let result = product_api
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

#[post("/product/delete/<id>")]
pub async fn delete_product(
    id: i64,
    product_api: &rocket::State<ProductBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = product_api.delete(id).await;
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
