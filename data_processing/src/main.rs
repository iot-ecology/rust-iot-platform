use crate::js_test::test_js;
use crate::storage_handler::handler_data_storage_string;
use common_lib::config::{get_config, read_config};
use common_lib::init_logger;
use common_lib::rabbit_utils::{get_rabbitmq_instance, init_rabbitmq_with_config};
use common_lib::redis_handler::{get_redis_instance, init_redis};
use futures_util::StreamExt;
use lapin::message::Delivery;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::{
    message::DeliveryResult,
    options::{BasicPublishOptions, QueueDeclareOptions},
};
use log::{error, info};
use quick_js::Context;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

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

    let channel = rabbit.connection.create_channel().await.unwrap();

    // let queue = channel
    //     .queue_declare(
    //         "pre_handler",
    //         QueueDeclareOptions::default(),
    //         FieldTable::default(),
    //     )
    //     .await.unwrap();
    // println!("Declared queue {:?}", queue);

    let mut consumer = channel
        .basic_consume(
            "pre_handler",
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();
    let guard = get_redis_instance().await.unwrap();

    println!("rmq consumer connected, waiting for messages");
    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                // delivery 是 Delivery 类型
                info!("received msg: {:?}", delivery);

                let result = String::from_utf8(delivery.data).unwrap();

                match handler_data_storage_string(
                    result,
                    Context::new().unwrap(),
                    guard1.influx_config.clone().unwrap(),
                    guard.clone(),
                )
                .await
                {
                    Ok(_) => {
                        info!("msg processed");
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                }

                // 在处理完消息后确认消息
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .unwrap();
            }
            Err(err) => {
                error!("Error receiving message: {:?}", err);
            }
        }
    }
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c signal");
}
