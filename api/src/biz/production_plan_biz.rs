use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{ProductionPlan, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;
pub struct ProductionPlanBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

#[async_trait::async_trait]
impl CrudOperations<ProductionPlan> for ProductionPlanBiz {
    async fn create(&self, item: ProductionPlan) -> Result<ProductionPlan, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(start_date) = item.start_date {
            updates.push(("start_date", start_date.to_string()));
        }

        if let Some(end_date) = item.end_date {
            updates.push(("end_date", end_date.to_string()));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        }

        log::info!("Creating production plan with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<ProductionPlan>(
            &self.mysql,
            "production_plans",
            updates,
        )
        .await;

        result
    }

    async fn update(&self, id: u64, item: ProductionPlan) -> Result<ProductionPlan, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(start_date) = item.start_date {
            updates.push(("start_date", start_date.to_string()));
        }

        if let Some(end_date) = item.end_date {
            updates.push(("end_date", end_date.to_string()));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        }

        log::info!("Updating production plan with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<ProductionPlan>(
            &self.mysql,
            "production_plans",
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
        log::info!("Deleting production plan with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "production_plans", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<ProductionPlan>, Error> {
        log::info!(
            "Fetching page of production plans with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<ProductionPlan>(
            &self.mysql,
            "production_plans",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<ProductionPlan>, Error> {
        log::info!(
            "Fetching list of production plans with filters: {:?}",
            filters
        );
        let result =
            common_lib::sql_utils::list::<ProductionPlan>(&self.mysql, "production_plans", filters)
                .await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<ProductionPlan, Error> {
        let result = common_lib::sql_utils::by_id_common::<ProductionPlan>(
            &self.mysql,
            "production_plans",
            id,
        )
        .await;
        result
    }
}
