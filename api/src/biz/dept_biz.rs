use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{Dept, Signal};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

#[derive(Debug)]
pub struct DeptBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

#[async_trait::async_trait]
impl CrudOperations<Dept> for DeptBiz {
    async fn create(&self, item: Dept) -> Result<Dept, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }
        if let Some(parent_id) = item.parent_id {
            updates.push(("parent_id", parent_id.to_string()));
        }

        log::info!("Creating Dept with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<Dept>(&self.mysql, "departments", updates).await;

        result
    }

    async fn update(&self, id: u64, item: Dept) -> Result<Dept, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }
        if let Some(parent_id) = item.parent_id {
            updates.push(("parent_id", parent_id.to_string()));
        }

        log::info!("Updating Dept with ID {}: {:?}", id, updates);

        let result =
            common_lib::sql_utils::update_by_id::<Dept>(&self.mysql, "departments", id, updates)
                .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting Dept with ID {}", id);
        common_lib::sql_utils::delete_by_id(&self.mysql, "departments", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<Dept>, Error> {
        log::info!(
            "Fetching page of Depts with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<Dept>(
            &self.mysql,
            "departments",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<Dept>, Error> {
        log::info!("Fetching list of Depts with filters: {:?}", filters);
        let result = common_lib::sql_utils::list::<Dept>(&self.mysql, "departments", filters).await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<Dept, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<Dept>(&self.mysql, "departments", id).await;
        result
    }
}
