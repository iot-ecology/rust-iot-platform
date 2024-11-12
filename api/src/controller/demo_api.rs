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
use std::ops::Deref;

const DB_KEY: &'static str = "items";

#[get("/")]
pub fn index(
    redis_op: &rocket::State<RedisOp>,
    mysql_op: &rocket::State<MysqlOp>,
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

    run_db_task(mysql_op.pool.clone());
    info!("{:?}", config);
    "Counter updated"
}
