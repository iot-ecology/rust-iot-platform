use crate::js_test::test_js;
use crate::storage_handler::handler_data_storage_string;
use common_lib::config::{get_config, read_config, Config};
use common_lib::init_logger;
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
    let guard = get_redis_instance().await.unwrap();

    let channel = rabbit.connection.create_channel().await.unwrap();

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

    pre_handler(guard1, guard, &connection, &channel1).await;
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c signal");
}

async fn pre_handler(
    guard1: MutexGuard<'_, Config>,
    guard: MutexGuard<'_, RedisWrapper>,
    rabbit_conn: &Connection,
    channel1: &Channel,
) {
    let mut consumer = channel1
        .basic_consume(
            "pre_handler",
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    info!("rmq consumer connected, waiting for messages");
    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                info!("received msg: {:?}", delivery);

                let result = String::from_utf8(delivery.data).unwrap();

                match handler_data_storage_string(
                    result,
                    Context::new().unwrap(),
                    guard1.influx_config.clone().unwrap(),
                    guard.clone(),
                    rabbit_conn,
                )
                .await
                {
                    Ok(_) => {
                        info!("msg processed");
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                };

                match channel1
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                {
                    Ok(_) => {
                        info!("消息已成功确认。");
                    }
                    Err(e) => {
                        error!("确认消息时发生错误: {}", e);
                        // 这里可以添加进一步的错误处理逻辑
                    }
                }
            }
            Err(err) => {
                error!("Error receiving message: {:?}", err);
            }
        }
    }
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
