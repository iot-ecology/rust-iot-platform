use crate::db::db_model::TcpHandler;
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{
    by_id_common, CrudOperations, Filter, PaginationParams, PaginationResult,
};
use r2d2;
use sqlx::MySqlPool;
pub struct TcpHandlerBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

#[async_trait::async_trait]
impl CrudOperations<TcpHandler> for TcpHandlerBiz {
    async fn create(&self, item: TcpHandler) -> Result<TcpHandler, Error> {
        let mut updates = vec![];

        if let Some(username) = item.username {
            updates.push(("username", username));
        } else {
            return Err(Error::msg("Username is required for tcp handler creation"));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        } else {
            return Err(Error::msg("Password is required for tcp handler creation"));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        } else {
            return Err(Error::msg("Name is required for tcp handler creation"));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        } else {
            return Err(Error::msg("Script is required for tcp handler creation"));
        }

        log::info!("Creating tcp handler with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<TcpHandler>(&self.mysql, "tcp_handlers", updates).await;

        result
    }

    async fn update(&self, id: u64, item: TcpHandler) -> Result<TcpHandler, Error> {
        let mut updates = vec![];

        if let Some(username) = item.username {
            updates.push(("username", username));
        } else {
            return Err(Error::msg("Username is required for tcp handler update"));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        } else {
            return Err(Error::msg("Password is required for tcp handler update"));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        } else {
            return Err(Error::msg("Name is required for tcp handler update"));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        } else {
            return Err(Error::msg("Script is required for tcp handler update"));
        }

        log::info!("Updating tcp handler with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<TcpHandler>(
            &self.mysql,
            "tcp_handlers",
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
        log::info!("Deleting tcp handler with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "tcp_handlers", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<TcpHandler>, Error> {
        log::info!(
            "Fetching page of tcp handlers with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<TcpHandler>(
            &self.mysql,
            "tcp_handlers",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<TcpHandler>, Error> {
        log::info!("Fetching list of tcp handlers with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<TcpHandler>(&self.mysql, "tcp_handlers", filters).await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<TcpHandler, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<TcpHandler>(&self.mysql, "tcp_handlers", id)
                .await;
        result
    }
}
