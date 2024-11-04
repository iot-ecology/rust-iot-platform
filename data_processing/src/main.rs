use common_lib::init_logger;
use common_lib::protocol_config::{get_config, read_config};
use common_lib::rabbit_utils::{get_rabbitmq_instance, init_rabbitmq_with_config};
use common_lib::redis_handler::{get_redis_instance, init_redis};
use lapin::options::BasicAckOptions;
use lapin::{
    message::DeliveryResult,
    options::{BasicPublishOptions, QueueDeclareOptions},
};
use log::{error, info};

mod js_test;
mod storage_handler;

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

    rabbit
        .consume2("queue1", |delivery| {
            // 返回一个异步块作为处理逻辑
            async move {
                let guard = get_redis_instance().await.unwrap();
                let x = guard.get_string("aaa").await.unwrap();
                info!("aa = {:?}", x);
                println!("received msg: {:?}", delivery);

                // 显式转换为 Box<dyn Error>
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "模拟的处理错误",
                )) as Box<dyn std::error::Error>)
            }
        })
        .await
        .unwrap();

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c signal");
}
async fn handle_message(d: DeliveryResult) {
    match d {
        Err(err) => error!("subscribe message error {err}"),
        Ok(data) => {
            if let Some(data) = data {
                let raw = data.data.clone();
                let ack = data.ack(BasicAckOptions::default());

                let instance = get_redis_instance();

                let redis = instance.await.expect("Error while awaiting message");

                let option = redis
                    .get_string("aaa")
                    .await
                    .expect("Error while getting message");

                info!("jaklsfjaklsfj = {:?}", option);

                info!(
                    "accept msg {}",
                    String::from_utf8(raw).expect("parse msg failed")
                );
                if let Err(err) = ack.await {
                    error!("ack failed {err}");
                }
            }
        }
    }
}
