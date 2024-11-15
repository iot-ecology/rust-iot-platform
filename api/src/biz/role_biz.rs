use crate::biz::repair_record_biz::RepairRecordBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{Role, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct RoleBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl RoleBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        RoleBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<Role> for RoleBiz {
    async fn create(&self, item: Role) -> Result<Role, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        if let Some(can_del) = item.can_del {
            updates.push(("can_del", can_del.to_string()));
        }

        log::info!("Creating role with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<Role>(&self.mysql, "roles", updates).await;

        result
    }

    async fn update(&self, id: u64, item: Role) -> Result<Role, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        if let Some(can_del) = item.can_del {
            updates.push(("can_del", can_del.to_string()));
        }

        log::info!("Updating role with ID {}: {:?}", id, updates);

        let result =
            common_lib::sql_utils::update_by_id::<Role>(&self.mysql, "roles", id, updates).await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting role with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "roles", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<Role>, Error> {
        log::info!(
            "Fetching page of roles with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result =
            common_lib::sql_utils::paginate::<Role>(&self.mysql, "roles", filters, pagination)
                .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<Role>, Error> {
        log::info!("Fetching list of roles with filters: {:?}", filters);
        let result = common_lib::sql_utils::list::<Role>(&self.mysql, "roles", filters).await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<Role, Error> {
        let result = common_lib::sql_utils::by_id_common::<Role>(&self.mysql, "roles", id).await;
        result
    }
}
