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
use chrono::NaiveDateTime;
use common_lib::config::Config;
use common_lib::mysql_utils::{run_db_task, MysqlOp};
use common_lib::rabbit_utils::RedisPool;
use common_lib::redis_pool_utils::RedisOp;
use log::info;
use r2d2;
use r2d2::Pool;
use r2d2_redis::redis::{Commands, RedisResult};
use r2d2_redis::RedisConnectionManager;
use rocket::http::{RawStr, Status};
use rocket::response::status;
use rocket::{get, State};
use sqlx::MySqlPool;
use std::ops::Deref;

#[get("/")]
pub async fn index(
    redis_op: &rocket::State<RedisOp>,
    mysql_op: &rocket::State<MySqlPool>,
    config: &State<Config>,
) -> &'static str {
    let result = redis_op.get_string("aaa");

    match result {
        Ok(value) => {
            println!("Value: {:?}", value);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };

    // 查询数据
    let rows = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(mysql_op.inner())
        .await
        .unwrap();

    for row in rows {
        println!("ID: {:?}, Name: {:?}", row.id, row.username);
    }

    info!("{:?}", config);
    "Counter updated"
}
#[get("/beat")]
pub async fn beat(
    redis_op: &rocket::State<RedisOp>,
    mysql_op: &rocket::State<MySqlPool>,
    config: &State<Config>,
) -> &'static str {
    "beat"
}
