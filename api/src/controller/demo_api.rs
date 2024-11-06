use crate::db::redis::RedisConnection;
use r2d2;
use r2d2::Pool;
use r2d2_redis::redis::{Commands, RedisResult};
use r2d2_redis::RedisConnectionManager;
use rocket::get;
use rocket::http::{RawStr, Status};
use rocket::response::status;
use std::ops::Deref;

const DB_KEY: &'static str = "items";

type RedisPool = Pool<RedisConnectionManager>;
#[get("/")]
pub fn index(pool: &rocket::State<RedisPool>) -> &'static str {
    // 使用 Redis 连接池执行操作
    increase_counter(pool.inner(), "counter");
    "Counter updated"
}

/// 执行 Redis 操作：增加计数器
fn increase_counter(pool: &r2d2::Pool<RedisConnectionManager>, key: &str) {
    let mut conn = pool.get().unwrap();
    let n: i64 = conn.incr(key, 1).unwrap();
    println!("Counter increased to {}", n);
}
