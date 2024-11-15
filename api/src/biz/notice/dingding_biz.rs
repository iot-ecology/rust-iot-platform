use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{DingDing, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;
pub struct DingDingBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

#[async_trait::async_trait]
impl CrudOperations<DingDing> for DingDingBiz {
    async fn create(&self, item: DingDing) -> Result<DingDing, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(access_token) = item.access_token {
            updates.push(("access_token", access_token));
        }

        if let Some(secret) = item.secret {
            updates.push(("secret", secret));
        }

        if let Some(content) = item.content {
            updates.push(("content", content));
        }

        log::info!("Creating DingDing with updates: {:?}", updates);

        let result = sql_utils::insert::<DingDing>(&self.mysql, "dingdings", updates).await;

        result
    }

    async fn update(&self, id: u64, item: DingDing) -> Result<DingDing, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(access_token) = item.access_token {
            updates.push(("access_token", access_token));
        }

        if let Some(secret) = item.secret {
            updates.push(("secret", secret));
        }

        if let Some(content) = item.content {
            updates.push(("content", content));
        }

        log::info!("Updating DingDing with ID {}: {:?}", id, updates);

        let result =
            sql_utils::update_by_id::<DingDing>(&self.mysql, "dingdings", id, updates).await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting DingDing with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "dingdings", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<DingDing>, Error> {
        log::info!(
            "Fetching page of DingDings with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result =
            sql_utils::paginate::<DingDing>(&self.mysql, "dingdings", filters, pagination).await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<DingDing>, Error> {
        log::info!("Fetching list of DingDings with filters: {:?}", filters);

        let result = sql_utils::list::<DingDing>(&self.mysql, "dingdings", filters).await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<DingDing, Error> {
        let result = sql_utils::by_id_common::<DingDing>(&self.mysql, "dingdings", id).await;
        result
    }
}
