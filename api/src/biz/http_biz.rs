use crate::biz::device_info_biz::DeviceInfoBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{HttpHandler, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use log::error;
use rocket::serde::json;
use sqlx::MySqlPool;

pub struct HttpHandlerBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl HttpHandlerBiz {
    pub fn setRedis(&self, ws: &HttpHandler) {
        self.redis.set_hash(
            "struct:Http",
            ws.device_info_id.unwrap().to_string().as_str(),
            <std::option::Option<std::string::String> as Clone>::clone(&ws.script)
                .unwrap()
                .as_str(),
        ).expect("TODO: panic message");
    }
    pub fn deleteRedis(&self, ws: &HttpHandler) {
        self.redis.delete_hash_field(
            "struct:Http",
            ws.device_info_id.unwrap().to_string().as_str(),
        ).expect("TODO: panic message");
    }
    pub async fn find_by_device_info_id(&self, device_info_id: u64) -> Result<Option<HttpHandler>, Error> {
        let sql = "select * from http_handlers where 1=1 and device_info_id = ?";

        let record = sqlx::query_as::<_, HttpHandler>(sql).bind(device_info_id.to_string())

            .fetch_optional(&self.mysql).await.with_context(|| {
            format!(
                "Failed to fetch updated record from table '{}' with username {:?}",
                "http_handlers",
                device_info_id
            )
        });

        match record {
            Ok(u) => Ok(u),
            Err(ee) => Err(ee),
        }
    }

    pub fn set_auth(&self, ws: &HttpHandler) {
        let result = json::to_string(ws);
        match result {
            Ok(o) => {
                let x = self
                    .redis
                    .set_hash(
                        "auth:http",
                        ws.device_info_id.unwrap().to_string().as_str(),
                        o.as_str(),
                    )
                    .unwrap();
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        HttpHandlerBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<HttpHandler> for HttpHandlerBiz {
    async fn create(&self, item: HttpHandler) -> Result<HttpHandler, Error> {
        let mut updates = vec![];

        if let Some(device_info_id) = item.device_info_id {
            updates.push(("device_info_id", device_info_id.to_string()));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(username) = item.username {
            updates.push(("username", username));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        log::info!("Creating HttpHandler with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<HttpHandler>(&self.mysql, "http_handlers", updates)
                .await;

        result
    }

    async fn update(&self, id: u64, item: HttpHandler) -> Result<HttpHandler, Error> {
        let mut updates = vec![];

        if let Some(device_info_id) = item.device_info_id {
            updates.push(("device_info_id", device_info_id.to_string()));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(username) = item.username {
            updates.push(("username", username));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        log::info!("Updating HttpHandler with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<HttpHandler>(
            &self.mysql,
            "http_handlers",
            id,
            updates,
        )
            .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: u64) -> Result<HttpHandler, Error> {
        log::info!("Deleting HttpHandler with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "http_handlers", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<HttpHandler>, Error> {
        log::info!(
            "Fetching page of HttpHandlers with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<HttpHandler>(
            &self.mysql,
            "http_handlers",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<HttpHandler>, Error> {
        log::info!("Fetching list of HttpHandlers with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<HttpHandler>(&self.mysql, "http_handlers", filters).await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<HttpHandler, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<HttpHandler>(&self.mysql, "http_handlers", id)
                .await;
        result
    }
}
