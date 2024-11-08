use crate::config::MqConfig;
use crate::redis_handler::get_redis_instance;
use futures_util::stream::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::{
    message::DeliveryResult,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use log::{debug, error, info};
use quick_js::Context;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard, OnceCell};

use serde_json::de::Read;

pub type RedisPool = Pool<RedisConnectionManager>;

pub struct RabbitMQ {
    pub connection: Connection,
    pub channel: Channel,
}

impl RabbitMQ {
    /// 创建新的 RabbitMQ 实例并建立连接
    ///
    /// # Arguments
    ///
    /// * `url` - RabbitMQ 服务器的 URL。
    ///
    /// # Returns
    ///
    /// 返回一个包含连接和频道的 `RabbitMQ` 实例。
    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let connection = Connection::connect(url, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        Ok(Self {
            connection,
            channel,
        })
    }

    /// 初始化队列和交换机
    ///
    /// # Arguments
    ///
    /// * `queue_name` - 要创建的队列名称。
    /// * `exchange_name` - 要创建的交换机名称。
    ///
    /// # Returns
    ///
    /// 返回一个空的 `Result`，如果成功则为 `Ok(())`。
    pub async fn setup(&self, queue_name: &str, exchange_name: &str) -> Result<(), Box<dyn Error>> {
        self.channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        self.channel
            .exchange_declare(
                exchange_name,
                ExchangeKind::Topic,
                lapin::options::ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(())
    }

    /// 绑定队列到交换机
    ///
    /// # Arguments
    ///
    /// * `queue_name` - 要绑定的队列名称。
    /// * `exchange_name` - 目标交换机名称。
    /// * `routing_key` - 用于路由的键。
    ///
    /// # Returns
    ///
    /// 返回一个空的 `Result`，如果成功则为 `Ok(())`。
    pub async fn bind_queue(
        &self,
        queue_name: &str,
        exchange_name: &str,
        routing_key: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.channel
            .queue_bind(
                queue_name,
                exchange_name,
                routing_key,
                lapin::options::QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(())
    }

    /// 发送消息
    ///
    /// # Arguments
    ///
    /// * `exchange_name` - 目标交换机名称。
    /// * `routing_key` - 消息的路由键。
    /// * `message` - 要发送的消息内容。
    ///
    /// # Returns
    ///
    /// 返回一个空的 `Result`，如果成功则为 `Ok(())`。
    pub async fn publish(
        &self,
        exchange_name: &str,
        routing_key: &str,
        message: &str,
    ) -> Result<(), Box<dyn Error>> {
        debug!("Publishing message: {}", message);
        self.channel
            .basic_publish(
                exchange_name,
                routing_key,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await?;
        Ok(())
    }

    pub async fn consume2<F, Fut>(&self, queue_name: &str, handler: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(lapin::message::Delivery, quick_js::Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + 'static,
    {
        let mut consumer = self
            .channel
            .basic_consume(
                queue_name,
                "",
                lapin::options::BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        // 消费消息并处理
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let delivery_tag = delivery.delivery_tag;

                // 在这里为每条消息创建独立的状态
                let context = Context::new().unwrap();

                // 调用传入的异步处理函数，传递独立的上下文
                match handler(delivery, context).await {
                    Ok(()) => {
                        // 处理成功，确认消息
                        self.channel
                            .basic_ack(delivery_tag, BasicAckOptions::default())
                            .await?;
                    }
                    Err(e) => {
                        eprintln!("Error handling message: {:?}", e);
                        // 处理失败，可以选择不确认消息或采取其他措施
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn consume<F>(&self, queue_name: &str, should_ack: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&[u8]) -> bool + Send + Sync + 'static,
    {
        let consumer = self
            .channel
            .basic_consume(
                queue_name,
                "",
                lapin::options::BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let should_ack = std::sync::Arc::new(should_ack);

        // 设置消费者的消息处理逻辑
        consumer.set_delegate(move |d: DeliveryResult| {
            let should_ack = Arc::clone(&should_ack);
            async move {
                match d {
                    Err(err) => error!("subscribe message error {err}"),
                    Ok(data) => {
                        if let Some(data) = data {
                            let raw = data.data.clone();
                            info!(
                                "accept msg {}",
                                String::from_utf8(raw.clone()).expect("parse msg failed")
                            );
                            if should_ack(&raw) {
                                if let Err(err) =
                                    data.ack(lapin::options::BasicAckOptions::default()).await
                                {
                                    error!("ack failed {err}");
                                }
                            } else {
                                info!("message not acknowledged");
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

static RABBIT_MQ_INSTANCE: OnceCell<Arc<Mutex<RabbitMQ>>> = OnceCell::const_new();
pub async fn init_rabbitmq(url: &str) -> Result<(), Box<dyn Error>> {
    let rabbit = RabbitMQ::new(url).await?;
    let rabbit_arc = Arc::new(Mutex::new(rabbit));
    RABBIT_MQ_INSTANCE
        .set(rabbit_arc)
        .map_err(|_| "RabbitMQ instance already initialized")?;
    Ok(())
}

pub async fn init_rabbitmq_with_config(config: MqConfig) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "amqp://{}:{}@{}:{}",
        config.username, config.password, config.host, config.port
    );
    let rabbit = RabbitMQ::new(url.as_str()).await?;
    let rabbit_arc = Arc::new(Mutex::new(rabbit));
    RABBIT_MQ_INSTANCE
        .set(rabbit_arc)
        .map_err(|_| "RabbitMQ instance already initialized")?;
    Ok(())
}

pub async fn get_rabbitmq_instance() -> Result<Arc<Mutex<RabbitMQ>>, Box<dyn Error>> {
    let instance = RABBIT_MQ_INSTANCE.get().ok_or("RabbitMQ not initialized")?;
    Ok(Arc::clone(instance))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rabbitmq_test() -> Result<(), Box<dyn Error>> {
        init_rabbitmq("amqp://guest:guest@localhost:5672").await?;

        let rabbit = get_rabbitmq_instance().await?;

        loop {
            rabbit
                .publish("", "queue1", "hello12")
                .await
                .expect("publish message failed");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        // rabbit.consume("queue1").await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
        Ok(())
    }
}
