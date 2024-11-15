use crate::biz::device_info_biz::DeviceInfoBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{HttpHandler, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct HttpHandlerBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl HttpHandlerBiz {
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

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting HttpHandler with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "http_handlers", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
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

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<HttpHandler>, Error> {
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
