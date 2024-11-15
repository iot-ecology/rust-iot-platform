use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{SimCard, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct SimCardBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

#[async_trait::async_trait]
impl CrudOperations<SimCard> for SimCardBiz {
    async fn create(&self, item: SimCard) -> Result<SimCard, Error> {
        let mut updates = vec![];

        updates.push(("access_number", item.access_number));
        updates.push(("iccid", item.iccid));
        updates.push(("imsi", item.imsi));
        updates.push(("operator", item.operator));

        if let Some(expiration) = item.expiration {
            updates.push(("expiration", expiration.to_string()));
        }

        log::info!("Creating sim card with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<SimCard>(&self.mysql, "sim_cards", updates).await;

        result
    }

    async fn update(&self, id: u64, item: SimCard) -> Result<SimCard, Error> {
        let mut updates = vec![];

        updates.push(("access_number", item.access_number));
        updates.push(("iccid", item.iccid));
        updates.push(("imsi", item.imsi));
        updates.push(("operator", item.operator));

        if let Some(expiration) = item.expiration {
            updates.push(("expiration", expiration.to_string()));
        }

        log::info!("Updating sim card with ID {}: {:?}", id, updates);

        let result =
            common_lib::sql_utils::update_by_id::<SimCard>(&self.mysql, "sim_cards", id, updates)
                .await;
        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting sim card with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "sim_cards", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<SimCard>, Error> {
        log::info!(
            "Fetching page of sim cards with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<SimCard>(
            &self.mysql,
            "sim_cards",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<SimCard>, Error> {
        log::info!("Fetching list of sim cards with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<SimCard>(&self.mysql, "sim_cards", filters).await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<SimCard, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<SimCard>(&self.mysql, "sim_cards", id).await;
        result
    }
}
