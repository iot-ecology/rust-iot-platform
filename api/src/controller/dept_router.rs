use crate::biz::dept_biz::DeptBiz;
use crate::db::db_model::{Dashboard, Dept};
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;
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
    if data.name.clone().is_none() {
        let error_json = json!({
            "code": 40000,
            "message": "操作失败",
            "data": "名称不能为空"
        });
        return Custom(Status::InternalServerError, Json(error_json));
    }

    match dept_api.create(data.into_inner()).await {
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

#[post("/Dept/update", format = "json", data = "<data>")]
pub async fn update_dept(
    data: Json<Dept>,
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = dept_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.name = data.name.clone();
            old.parent_id = data.parent_id.clone();
            let result = dept_api.update(old.id.unwrap(), old).await;
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

#[get("/Dept/page?<page>&<page_size>&<pid>&<name>")]
pub async fn page_dept(
    page: Option<u64>,
    name: Option<String>,
    pid: Option<u64>,
    page_size: Option<u64>,
    dept_api: &rocket::State<DeptBiz>,
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
            value: name.unwrap_or_else(String::new),
            operation: FilterOperation::AllLike,
            value2: None,
        },
        FilterInfo {
            field: "parent_id".to_string(),
            value: pid.unwrap_or_default().to_string(),
            operation: FilterOperation::Equal,
            value2: None,
        },
    ];

    let result = dept_api
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
            error!("query error = {:?}", e);

            let error_json = json!({
                               "code": 40000,

                    "message": "查询失败"

            });
            return Custom(Status::Ok, Json(error_json));
        }
    }
}

#[post("/Dept/delete/<id>")]
pub async fn delete_dept(
    id: u64,
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = dept_api.delete(id).await;
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

#[get("/Dept/subs?<id>")]
pub async fn find_by_id_subs(
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
    id: u64,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = dept_api
        .list(vec![FilterInfo {
            field: "parent_id".to_string(),
            value: id.to_string(),
            operation: FilterOperation::Equal,
            value2: None,
        }])
        .await;
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
            return Custom(Status::Ok, Json(success_json));
        }
    }
}

#[get("/Dept/list")]
pub async fn list_dept(
    dept_api: &rocket::State<DeptBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = dept_api.list(Vec::new()).await;
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
