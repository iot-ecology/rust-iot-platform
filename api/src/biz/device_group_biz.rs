use crate::biz::dept_biz::DeptBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{DeviceGroup, Signal};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

#[derive(Debug)]
pub struct DeviceGroupBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl DeviceGroupBiz {
    pub async fn find_by_name(&self, username: Option<String>) -> Result<Option<DeviceGroup>, Error> {
        if username.is_none() {
            return Ok(None);
        }

        let sql = "select * from device_groups where name = ?";

        let record = sqlx::query_as::<_, DeviceGroup>(sql)
            .bind(username.clone().unwrap())
            .fetch_optional(&self.mysql)
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch updated record from table '{}' with username {:?}",
                    "users",
                    username.clone()
                )
            });

        return match record {
            Ok(u) => Ok(u),
            Err(ee) => Err(ee),
        };
    }

    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        DeviceGroupBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<DeviceGroup> for DeviceGroupBiz {
    async fn create(&self, item: DeviceGroup) -> Result<DeviceGroup, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        log::info!("Creating DeviceGroup with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<DeviceGroup>(&self.mysql, "device_groups", updates)
                .await;

        result
    }

    async fn update(&self, id: i64, item: DeviceGroup) -> Result<DeviceGroup, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        log::info!("Updating DeviceGroup with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<DeviceGroup>(
            &self.mysql,
            "device_groups",
            id,
            updates,
        )
            .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<DeviceGroup, Error> {
        log::info!("Deleting DeviceGroup with ID {}", id);
        common_lib::sql_utils::delete_by_id(&self.mysql, "device_groups", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<DeviceGroup>, Error> {
        log::info!(
            "Fetching page of DeviceGroups with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<DeviceGroup>(
            &self.mysql,
            "device_groups",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<DeviceGroup>, Error> {
        log::info!("Fetching list of DeviceGroups with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<DeviceGroup>(&self.mysql, "device_groups", filters).await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<DeviceGroup, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<DeviceGroup>(&self.mysql, "device_groups", id)
                .await;
        result
    }
}
