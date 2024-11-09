use common_lib::init_logger;
use common_lib::models::MQTTMessage;
use common_lib::rabbit_utils::{get_rabbitmq_instance, RabbitMQ};
use log::{debug, error, info};
use once_cell::sync::OnceCell;
use rumqttc::{AsyncClient, ConnectionError, Event, Incoming, MqttOptions, QoS};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::MutexGuard;
use tokio::{task, time};

static CLIENTS: OnceCell<Arc<Mutex<HashMap<String, AsyncClient>>>> = OnceCell::new();

pub fn init_mqtt_map() -> Result<(), Box<dyn Error>> {
    let clients = Arc::new(Mutex::new(HashMap::new()));
    CLIENTS.set(clients).unwrap();
    Ok(())
}

pub fn put_client(name: String, client: AsyncClient) {
    let mut clients_lock = CLIENTS.get().unwrap().lock().unwrap();
    clients_lock.insert(name, client);
}

pub fn get_client(client_name: &str) -> Option<AsyncClient> {
    let clients_lock = CLIENTS.get().unwrap().lock().unwrap();
    info!("size = {}", clients_lock.len());
    clients_lock.keys().for_each(|key| {
        info!("client_name = {}", key);
    });
    clients_lock.get(client_name).cloned()
}

pub async fn create_client(
    client_name: &str,
    topic: &str,
    username: &str,
    password: &str,
    ip: &str,
    port: u16,
) -> Result<(AsyncClient, rumqttc::EventLoop), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new(client_name, ip, port);
    // mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_credentials(username, password);
    let (client, eventloop) = AsyncClient::new(mqttoptions.clone(), 10);
    client.subscribe(topic, QoS::AtMostOnce).await?;

    let mut clients_lock = CLIENTS.get().unwrap().lock().unwrap();
    clients_lock.insert(client_name.to_string(), client.clone());

    Ok((client, eventloop))
}

async fn disconnect_client(client_name: String) {
    if let Some(client) = {
        let mut clients_lock = CLIENTS.get().unwrap().lock().unwrap();
        clients_lock.remove(&client_name)
    } {
        client.disconnect().await.unwrap();
        info!("Disconnected client: {}", client_name);
    }
}
use lapin::{Channel, Connection};

pub async fn handler_event(
    event: &Result<Event, ConnectionError>,
    topic: &str,
    client_name: &str,
) -> Option<Result<(), Box<dyn Error>>> {
    let rabbitmq_instance = match get_rabbitmq_instance().await {
        Ok(instance) => instance,
        Err(e) => {
            error!("Failed to get RabbitMQ instance: {:?}", e);
            return Some(Err(e));
        }
    };
    let mut rabbitmq = rabbitmq_instance.lock().await;

    // 创建 channel
    let channel = match rabbitmq.connection.create_channel().await {
        Ok(ch) => ch,
        Err(e) => {
            error!("Failed to create channel: {:?}", e);
            return Some(Err(Box::new(e)));
        }
    };

    // 匹配不同事件类型
    match event {
        Ok(Event::Incoming(Incoming::Publish(publish))) => {
            let payload_str =
                std::str::from_utf8(&publish.payload).unwrap_or_else(|_| "<Invalid UTF-8>");
            info!(
                "Received message on client_name = {} topic = {}: message = {:?}",
                client_name, topic, payload_str
            );

            let mqtt_msg = MQTTMessage {
                mqtt_client_id: client_name.to_string(),
                message: payload_str.to_string(),
            };

            if let Err(e) = channel
                .basic_publish(
                    "",
                    "pre_handler",
                    lapin::options::BasicPublishOptions::default(),
                    mqtt_msg.to_json_string().as_bytes(),
                    lapin::BasicProperties::default(),
                )
                .await
            {
                error!("Failed to publish message: {:?}", e);
                return Some(Err(Box::new(e))); // 发布消息失败时返回 Some(Err(...))
            }
        }
        Ok(Event::Incoming(Incoming::ConnAck(connack))) => {
            debug!("Connection Acknowledged: {:?}", connack);
        }
        Ok(Event::Incoming(Incoming::SubAck(suback))) => {
            info!(
                "Subscribe Acknowledged: pkid={}, return_codes={:?}",
                suback.pkid, suback.return_codes
            );
        }
        Ok(Event::Incoming(Incoming::PingResp)) => {
            info!("Ping Response received");
        }
        Ok(v) => {
            debug!("Other Event = {:?}", v);
        }
        Err(e) => {
            error!("Error = {:?}", e);
            return Some(Err(Box::from(e.to_string())));
        }
    }
    Some(Ok(())) // 执行完成返回 Some(Ok(()))
}
pub async fn handler_event2(
    event: &Result<Event, ConnectionError>,
    topic: &str,
    client_name: &str,
) -> Option<Result<(), Box<dyn Error>>> {
    match event {
        Ok(Event::Incoming(Incoming::Publish(publish))) => {
            let payload_str =
                std::str::from_utf8(&publish.payload).unwrap_or_else(|_| "<Invalid UTF-8>");
            info!(
                "Received message on client_name = {} topic = {}: message = {:?}",
                client_name.clone(),
                topic,
                payload_str
            );

            let mqttMsg = MQTTMessage {
                mqtt_client_id: client_name.to_string(),
                message: payload_str.to_string(),
            };
        }
        Ok(Event::Incoming(Incoming::ConnAck(connack))) => {
            debug!("Connection Acknowledged: {:?}", connack);
        }
        Ok(Event::Incoming(Incoming::SubAck(suback))) => {
            info!(
                "Subscribe Acknowledged: pkid={}, return_codes={:?}",
                suback.pkid, suback.return_codes
            );
        }
        Ok(Event::Incoming(Incoming::PingResp)) => {
            info!("Ping Response received");
        }
        Ok(v) => {
            debug!("Other Event = {:?}", v);
        }
        Err(e) => {
            error!("Error = {:?}", e);
            return Some(Ok(()));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use tokio::time::{self, Duration};

    #[tokio::test]
    async fn test_mqtt_client_connections() -> Result<(), Box<dyn Error>> {
        // 初始化 logger 和 MQTT 映射
        init_logger();
        init_mqtt_map()?;

        // 创建两个 MQTT 客户端
        let (client1, mut eventloop1) =
            create_client("test-1", "/tt1", "admin", "public", "localhost", 1883).await?;
        let (client2, mut eventloop2) =
            create_client("test-2", "/tt2", "admin", "public", "localhost", 1883).await?;

        let client_name_clone = "test-1".to_string();
        // 模拟 10 秒后断开 client1
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(10)).await;
            disconnect_client(client_name_clone).await;
        });

        // 启动事件循环任务
        // tokio::spawn(event_loop("/tt1".to_string(), eventloop1,"".to_string()));
        // tokio::spawn(event_loop("/tt2".to_string(), eventloop2,"".to_string()));

        // 等待 ctrl-c 信号
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl-c signal");

        Ok(())
    }
}

pub async fn event_loop(topic: String, mut eventloop: rumqttc::EventLoop, client_name: String) {
    loop {
        match eventloop.poll().await {
            Ok(event) => {
                // 处理事件
                if let Some(Err(e)) = handler_event(&Ok(event), &topic, &client_name).await {
                    error!("Error handling event: {:?}", e);
                }
            }
            Err(e) => {
                error!(
                    "Error polling eventloop for client_name = {} topic = {}: {:?}",
                    client_name, topic, e
                );
                // 可以根据需要在此处增加重连逻辑
            }
        }
    }
}

async fn requests(client: AsyncClient) {
    client
        .subscribe("hello/world", QoS::AtMostOnce)
        .await
        .unwrap();
    for _ in 0..10 {
        client
            .publish("hello/world", QoS::ExactlyOnce, false, "hello")
            .await
            .unwrap();
        time::sleep(Duration::from_secs(1)).await;
    }
    time::sleep(Duration::from_secs(120)).await;
}
