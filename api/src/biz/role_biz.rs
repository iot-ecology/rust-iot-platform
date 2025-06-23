/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::biz::repair_record_biz::RepairRecordBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{Role, Signal, User, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct RoleBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl RoleBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        RoleBiz { redis, mysql }
    }

    pub async fn find_by_name(&self, username: Option<String>) -> Result<Option<Role>, Error> {
        if username.is_none() {
            return Ok(None);
        }

        let sql = "select * from roles where name = ?";

        let record = sqlx::query_as::<_, Role>(sql)
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

    async fn update(&self, id: i64, item: Role) -> Result<Role, Error> {
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

    async fn delete(&self, id: i64) -> Result<Role, Error> {
        log::info!("Deleting role with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "roles", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
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

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<Role>, Error> {
        log::info!("Fetching list of roles with filters: {:?}", filters);
        let result = common_lib::sql_utils::list::<Role>(&self.mysql, "roles", filters).await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<Role, Error> {
        let result = common_lib::sql_utils::by_id_common::<Role>(&self.mysql, "roles", id).await;
        result
    }
}
