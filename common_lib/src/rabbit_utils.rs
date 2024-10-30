use crate::protocol_config::MqConfig;
use lapin::{
    message::DeliveryResult,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use log::{debug, error, info};
use std::error::Error;
use tokio::sync::{Mutex, MutexGuard, OnceCell};

pub struct RabbitMQ {
    connection: Connection,
    channel: Channel,
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

    /// 开始消费消息
    ///
    /// # Arguments
    ///
    /// * `queue_name` - 要消费消息的队列名称。
    ///
    /// # Returns
    ///
    /// 返回一个空的 `Result`，如果成功则为 `Ok(())`。
    pub async fn consume(&self, queue_name: &str) -> Result<(), Box<dyn Error>> {
        let consumer = self
            .channel
            .basic_consume(
                queue_name,
                "",
                lapin::options::BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        // 设置消费者的消息处理逻辑
        consumer.set_delegate(|d: DeliveryResult| async move {
            match d {
                Err(err) => error!("subscribe message error {err}"),
                Ok(data) => {
                    if let Some(data) = data {
                        let raw = data.data.clone();
                        let ack = data.ack(lapin::options::BasicAckOptions::default());
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
        });

        Ok(())
    }
}

static RABBIT_MQ_INSTANCE: OnceCell<Mutex<RabbitMQ>> = OnceCell::const_new();

pub async fn init_rabbitmq(url: &str) -> Result<(), Box<dyn Error>> {
    let rabbit = RabbitMQ::new(url).await?;
    RABBIT_MQ_INSTANCE
        .set(Mutex::new(rabbit))
        .map_err(|_| "RabbitMQ instance already initialized")?;
    Ok(())
}

pub async fn init_rabbitmq_with_config(config: MqConfig) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "amqp://{}:{}@{}:{}",
        config.username, config.password, config.host, config.port
    );
    let rabbit = RabbitMQ::new(url.as_str()).await?;
    RABBIT_MQ_INSTANCE
        .set(Mutex::new(rabbit))
        .map_err(|_| "RabbitMQ instance already initialized")?;
    Ok(())
}

pub async fn get_rabbitmq_instance() -> Result<MutexGuard<'static, RabbitMQ>, Box<dyn Error>> {
    let instance = RABBIT_MQ_INSTANCE.get().ok_or("RabbitMQ not initialized")?;
    Ok(instance.lock().await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rabbitmq_test() -> Result<(), Box<dyn Error>> {
        init_rabbitmq("amqp://guest:guest@localhost:5672").await?;

        let rabbit = get_rabbitmq_instance().await?;

        rabbit
            .publish("", "queue1", "hello12")
            .await
            .expect("publish message failed");
        // rabbit.consume("queue1").await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        Ok(())
    }
}
