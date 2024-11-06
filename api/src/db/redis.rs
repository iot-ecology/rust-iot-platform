use common_lib::config::{get_config, read_config};
use common_lib::models::{Auth, HttpMessage};
use common_lib::rabbit_utils::{get_rabbitmq_instance, init_rabbitmq_with_config};
use common_lib::redis_handler::{get_redis_instance, init_redis, RedisWrapper};
use log::info;
use r2d2_redis::RedisConnectionManager;
use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{request, State};
use serde_json::{from_str, json};
use std::sync::Mutex;
use tokio::runtime::Runtime;
use tokio::sync::MutexGuard;

use common_lib::config::RedisConfig;

pub struct RedisConnection(pub r2d2::PooledConnection<RedisConnectionManager>);
