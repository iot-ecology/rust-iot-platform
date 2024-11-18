use crate::biz::tcp_biz::TcpHandlerBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::WebSocketHandler;
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct WebSocketHandlerBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl WebSocketHandlerBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        WebSocketHandlerBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<WebSocketHandler> for WebSocketHandlerBiz {
    async fn create(&self, item: WebSocketHandler) -> Result<WebSocketHandler, Error> {
        let mut updates = vec![];

        if let Some(device_info_id) = item.device_info_id {
            updates.push(("device_info_id", device_info_id.to_string()));
        } else {
            return Err(Error::msg(
                "DeviceInfoId is required for WebSocketHandler creation",
            ));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        } else {
            return Err(Error::msg("Name is required for WebSocketHandler creation"));
        }

        if let Some(username) = item.username {
            updates.push(("username", username));
        } else {
            return Err(Error::msg(
                "Username is required for WebSocketHandler creation",
            ));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        } else {
            return Err(Error::msg(
                "Password is required for WebSocketHandler creation",
            ));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        } else {
            return Err(Error::msg(
                "Script is required for WebSocketHandler creation",
            ));
        }

        log::info!("Creating WebSocketHandler with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<WebSocketHandler>(
            &self.mysql,
            "websocket_handlers",
            updates,
        )
        .await;
        result
    }

    async fn update(&self, id: u64, item: WebSocketHandler) -> Result<WebSocketHandler, Error> {
        let mut updates = vec![];

        if let Some(device_info_id) = item.device_info_id {
            updates.push(("device_info_id", device_info_id.to_string()));
        } else {
            return Err(Error::msg(
                "DeviceInfoId is required for WebSocketHandler update",
            ));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        } else {
            return Err(Error::msg("Name is required for WebSocketHandler update"));
        }

        if let Some(username) = item.username {
            updates.push(("username", username));
        } else {
            return Err(Error::msg(
                "Username is required for WebSocketHandler update",
            ));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        } else {
            return Err(Error::msg(
                "Password is required for WebSocketHandler update",
            ));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        } else {
            return Err(Error::msg("Script is required for WebSocketHandler update"));
        }

        log::info!("Updating WebSocketHandler with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<WebSocketHandler>(
            &self.mysql,
            "websocket_handlers",
            id,
            updates,
        )
        .await;
        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting WebSocketHandler with ID {}", id);
        common_lib::sql_utils::delete_by_id(&self.mysql, "websocket_handlers", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<WebSocketHandler>, Error> {
        log::info!(
            "Fetching page of WebSocketHandlers with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );
        let result = common_lib::sql_utils::paginate::<WebSocketHandler>(
            &self.mysql,
            "websocket_handlers",
            filters,
            pagination,
        )
        .await;
        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<WebSocketHandler>, Error> {
        log::info!(
            "Fetching list of WebSocketHandlers with filters: {:?}",
            filters
        );
        let result = common_lib::sql_utils::list::<WebSocketHandler>(
            &self.mysql,
            "websocket_handlers",
            filters,
        )
        .await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<WebSocketHandler, Error> {
        let result = common_lib::sql_utils::by_id_common::<WebSocketHandler>(
            &self.mysql,
            "websocket_handlers",
            id,
        )
        .await;
        result
    }
}