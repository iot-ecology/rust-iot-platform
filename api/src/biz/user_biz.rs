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

use crate::db::db_model::User;
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{
    by_id_common, CrudOperations, FilterInfo, PaginationParams, PaginationResult,
};
use r2d2;
use sqlx::MySqlPool;

pub struct UserBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

impl UserBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        UserBiz { redis, mysql }
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Result<User, Error> {
        let user = self.query_mysql_for_user(user_id).await;
        user
    }

    async fn query_mysql_for_user(&self, user_id: i64) -> Result<User, Error> {
        let x = self.by_id(user_id).await;
        x
    }

    pub async fn find_user_with_pwd(&self, username: String, password: String) -> Option<User> {
        let sql = "select * from users where username = ? and password = ?";
        let record = sqlx::query_as::<_, User>(sql)
            .bind(username.clone())
            .bind(password.clone())
            .fetch_optional(&self.mysql)
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch updated record from table '{}' with username {:?}",
                    "users",
                    username.clone()
                )
            });

        record.unwrap()
    }

    pub async fn find_by_username(&self, username: Option<String>) -> Result<Option<User>, Error> {
        if username.is_none() {
            return Ok(None);
        }

        let sql = "select * from users where username = ?";

        let record = sqlx::query_as::<_, User>(sql)
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
impl CrudOperations<User> for UserBiz {
    async fn create(&self, item: User) -> Result<User, Error> {
        let mut updates = vec![];

        if let Some(username) = item.username {
            updates.push(("username", username));
        } else {
            return Err(Error::msg("Username is required for user creation"));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        } else {
            return Err(Error::msg("Password is required for user creation"));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        } else {
            return Err(Error::msg("Status is required for user creation"));
        }

        if let Some(email) = item.email {
            updates.push(("email", email));
        } else {
            return Err(Error::msg("Email is required for user creation"));
        }

        log::info!("Creating user with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<User>(&self.mysql, "users", updates).await;

        result
    }
    async fn update(&self, id: i64, item: User) -> Result<User, Error> {
        let mut updates = vec![];

        if let Some(username) = item.username {
            updates.push(("username", username));
        } else {
            return Err(Error::msg("Username is required for user update"));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        } else {
            return Err(Error::msg("Password is required for user update"));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        } else {
            return Err(Error::msg("Status is required for user update"));
        }

        if let Some(email) = item.email {
            updates.push(("email", email));
        } else {
            return Err(Error::msg("Email is required for user update"));
        }

        log::info!("Updating user with ID {}: {:?}", id, updates);

        let result =
            common_lib::sql_utils::update_by_id::<User>(&self.mysql, "users", id, updates).await;
        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<User, Error> {
        log::info!("Deleting user with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "users", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<User>, Error> {
        log::info!(
            "Fetching page of users with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result =
            common_lib::sql_utils::paginate::<User>(&self.mysql, "users", filters, pagination)
                .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<User>, Error> {
        log::info!("Fetching list of users with filters: {:?}", filters);
        let result = common_lib::sql_utils::list::<User>(&self.mysql, "users", filters).await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<User, Error> {
        let result = common_lib::sql_utils::by_id_common::<User>(&self.mysql, "users", id).await;
        result
    }
}
