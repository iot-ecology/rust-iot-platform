use crate::biz::signal_biz::SignalBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{SignalDelayWaring, SimCard, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct SignalDelayWaringBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl SignalDelayWaringBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        SignalDelayWaringBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<SignalDelayWaring> for SignalDelayWaringBiz {
    async fn create(&self, item: SignalDelayWaring) -> Result<SignalDelayWaring, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        log::info!("Creating signal delay warning with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            updates,
        )
        .await;

        result
    }

    async fn update(&self, id: u64, item: SignalDelayWaring) -> Result<SignalDelayWaring, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        log::info!(
            "Updating signal delay warning with ID {}: {:?}",
            id,
            updates
        );

        let result = common_lib::sql_utils::update_by_id::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            id,
            updates,
        )
        .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: u64) -> Result<SignalDelayWaring, Error> {
        log::info!("Deleting signal delay warning with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "signal_delay_warings", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<SignalDelayWaring>, Error> {
        log::info!(
            "Fetching page of signal delay warnings with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<SignalDelayWaring>, Error> {
        log::info!(
            "Fetching list of signal delay warnings with filters: {:?}",
            filters
        );
        let result = common_lib::sql_utils::list::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            filters,
        )
        .await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<SignalDelayWaring, Error> {
        let result = common_lib::sql_utils::by_id_common::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            id,
        )
        .await;
        result
    }
}
