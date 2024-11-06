use common_lib::rabbit_utils::RedisPool;
use common_lib::redis_pool_utils::RedisOp;
use r2d2;
use r2d2::Pool;
use r2d2_redis::redis::{Commands, RedisResult};
use r2d2_redis::RedisConnectionManager;
use rocket::get;
use rocket::http::{RawStr, Status};
use rocket::response::status;
use std::ops::Deref;

const DB_KEY: &'static str = "items";

#[get("/")]
pub fn index(pool: &rocket::State<RedisOp>) -> &'static str {
    "Counter updated"
}
