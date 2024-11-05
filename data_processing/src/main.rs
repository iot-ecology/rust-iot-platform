use crate::calc_handler::calc_handler_mq;
use crate::js_test::test_js;
use crate::storage_handler::{handler_data_storage_string, pre_handler};
use crate::waring_dealy_handler::{handler_waring_delay_string, waring_delay_handler};
use crate::waring_handler::{handler_waring_string, waring_handler};
use common_lib::config::{get_config, read_config, Config, InfluxConfig};
use common_lib::init_logger;
use common_lib::mongo_utils::{get_mongo, init_mongo, MongoDBManager};
use common_lib::rabbit_utils::{get_rabbitmq_instance, init_rabbitmq_with_config, RabbitMQ};
use common_lib::redis_handler::{get_redis_instance, init_redis, RedisWrapper};
use futures_util::StreamExt;
use lapin::message::Delivery;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::{
    message::DeliveryResult,
    options::{BasicPublishOptions, QueueDeclareOptions},
    Channel, Connection, ConnectionProperties, Error as LapinError, Result as LapinResult,
};
use log::{error, info};
use quick_js::Context;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

mod calc_handler;
mod js_test;
mod storage_handler;
mod waring_dealy_handler;
mod waring_handler;

#[tokio::main]
async fn main() {
    init_logger();
    read_config("app-local.yml").await.unwrap();
    let guard1 = get_config().await.unwrap();
    init_redis(guard1.redis_config.clone()).await.unwrap();
    init_rabbitmq_with_config(guard1.mq_config.clone())
        .await
        .unwrap();
    let rabbit = get_rabbitmq_instance().await.unwrap();
    let redis_wrapper = get_redis_instance().await.unwrap();

    let channel = rabbit.connection.create_channel().await.unwrap();
    let mongo_config = guard1.mongo_config.clone().unwrap();

    init_mongo(mongo_config.clone()).await.unwrap();

    let mongoConfig = guard1.mongo_config.clone().unwrap();
    let option = guard1.influx_config.clone().unwrap();

    let mongo_manager_wrapper = get_mongo().await.unwrap();
    ensure_queue_exists(&channel, "calc_queue").await;
    ensure_queue_exists(&channel, "waring_handler").await;
    ensure_queue_exists(&channel, "waring_notice").await;
    ensure_queue_exists(&channel, "transmit_handler").await;
    ensure_queue_exists(&channel, "waring_delay_handler").await;
    ensure_queue_exists(&channel, "pre_handler").await;
    ensure_queue_exists(&channel, "pre_tcp_handler").await;
    ensure_queue_exists(&channel, "pre_http_handler").await;
    ensure_queue_exists(&channel, "pre_ws_handler").await;
    ensure_queue_exists(&channel, "pre_coap_handler").await;

    let url = format!(
        "amqp://{}:{}@{}:{}",
        guard1.mq_config.username,
        guard1.mq_config.password,
        guard1.mq_config.host,
        guard1.mq_config.port
    );

    let connection = Connection::connect(url.as_str(), ConnectionProperties::default())
        .await
        .unwrap();

    let channel1 = connection.create_channel().await.unwrap();
    let redis = redis_wrapper.clone();

    // pre_handler(guard1, guard, &connection, &channel1).await;
    // waring_handler(option, wrapper, &connection, &channel1, mongoConfig.waring_collection.unwrap()).await;

    let (pre_result, waring_result, waring_dealy_handler, calc_handler_mq) = tokio::join!(
        pre_handler(guard1, redis_wrapper, &connection, &channel1),
        waring_handler(
            option.clone(),
            redis.clone(),
            &connection,
            &channel1,
            mongoConfig.waring_collection.clone().unwrap(),
            &mongo_manager_wrapper
        ),
        waring_delay_handler(
            option.clone(),
            redis.clone(),
            &connection,
            &channel1,
            mongoConfig.script_waring_collection.clone().unwrap(),
            &mongo_manager_wrapper
        ),
        calc_handler_mq(
            option.clone(),
            redis.clone(),
            &connection,
            &channel1,
            mongoConfig.collection.clone().unwrap(),
            &mongo_manager_wrapper
        )
    );

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c signal");
}

async fn ensure_queue_exists(channel: &Channel, queue_name: &str) -> bool {
    // 尝试声明队列，如果队列已存在，则返回 Ok(true)
    let result = channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions {
                passive: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await;

    match result {
        Ok(res) => true,
        Err(error) => false,
    }
}
