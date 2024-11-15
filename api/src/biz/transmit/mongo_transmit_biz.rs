use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{MongoTransmit, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;
pub struct MongoTransmitBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

#[async_trait::async_trait]
impl CrudOperations<MongoTransmit> for MongoTransmitBiz {
    async fn create(&self, item: MongoTransmit) -> Result<MongoTransmit, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(host) = item.host {
            updates.push(("host", host.to_string()));
        }

        if let Some(username) = item.username {
            updates.push(("username", username.to_string()));
        }

        if let Some(password) = item.password {
            updates.push(("password", password.to_string()));
        }

        if let Some(port) = item.port {
            updates.push(("port", port.to_string()));
        }

        log::info!("Creating MongoTransmit with updates: {:?}", updates);

        let result =
            sql_utils::insert::<MongoTransmit>(&self.mysql, "mongo_transmits", updates).await;

        result
    }

    async fn update(&self, id: u64, item: MongoTransmit) -> Result<MongoTransmit, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(host) = item.host {
            updates.push(("host", host.to_string()));
        }

        if let Some(username) = item.username {
            updates.push(("username", username.to_string()));
        }

        if let Some(password) = item.password {
            updates.push(("password", password.to_string()));
        }

        if let Some(port) = item.port {
            updates.push(("port", port.to_string()));
        }

        log::info!("Updating MongoTransmit with ID {}: {:?}", id, updates);

        let result =
            sql_utils::update_by_id::<MongoTransmit>(&self.mysql, "mongo_transmits", id, updates)
                .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting MongoTransmit with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "mongo_transmits", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<MongoTransmit>, Error> {
        log::info!(
            "Fetching page of MongoTransmits with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<MongoTransmit>(
            &self.mysql,
            "mongo_transmits",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<MongoTransmit>, Error> {
        log::info!(
            "Fetching list of MongoTransmits with filters: {:?}",
            filters
        );

        let result =
            sql_utils::list::<MongoTransmit>(&self.mysql, "mongo_transmits", filters).await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<MongoTransmit, Error> {
        let result =
            sql_utils::by_id_common::<MongoTransmit>(&self.mysql, "mongo_transmits", id).await;
        result
    }
}
