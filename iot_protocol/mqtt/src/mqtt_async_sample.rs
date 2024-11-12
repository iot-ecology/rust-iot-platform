use common_lib::init_logger;
use common_lib::models::MQTTMessage;
use common_lib::rabbit_utils::{get_rabbitmq_instance, RabbitMQ};
use log::{debug, error, info};
use once_cell::sync::OnceCell;
use rumqttc::{AsyncClient, ConnectionError, Event, Incoming, MqttOptions, Outgoing, QoS};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::{broadcast, MutexGuard};
use tokio::task::JoinHandle;
use tokio::{task, time};

pub struct MqttClientState {
    client: AsyncClient,
    shutdown_tx: broadcast::Sender<()>,
    eventloop_handle: JoinHandle<()>,
}

lazy_static::lazy_static! {
    static ref CLIENT_MANAGER: Arc<Mutex<HashMap<String, MqttClientState>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn shutdown_client(client_id: &str) -> Result<(), Box<dyn Error>> {
    let mut clients = CLIENT_MANAGER.lock();

    if let Some(state) = clients?.remove(client_id) {
        state.client.disconnect().await?;

        state.shutdown_tx.send(())?;

        state.eventloop_handle.await?;

        info!("Client {} successfully shut down", client_id);
        Ok(())
    } else {
        Err("Client not found".into())
    }
}
pub async fn create_client(
    client_name: &str,
    topic: &str,
    username: &str,
    password: &str,
    ip: &str,
    port: u16,
) -> Result<(AsyncClient,), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new(client_name, ip, port);
    mqttoptions.set_credentials(username, password);
    let (client, mut eventloop) = AsyncClient::new(mqttoptions.clone(), 10);
    client.subscribe(topic, QoS::AtMostOnce).await?;
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);

    // 将 `client_name` 和 `topic` 转换为 `String`
    let client_name = client_name.to_string();
    let topic = topic.to_string();

    // Clone the client_name to move into the async task
    let client_name_clone = client_name.clone();

    let eventloop_handle = tokio::spawn({
        let client = client.clone();
        async move {
            loop {
                tokio::select! {
                                _ = shutdown_rx.recv() => {
                                    info!("Shutdown signal received, stopping event loop");
                                    break;
                                }
                                notification = eventloop.poll() => {
                                    match notification {
                                        Ok(event) => {
                                            match event {
                                                Event::Incoming(incoming) => {
                                                    match incoming {
                                                        Incoming::Publish(publish) => {
                                                            let payload_str = std::str::from_utf8(&publish.payload)
                                                                .unwrap_or_else(|_| "<Invalid UTF-8>");
                                                            info!(
                                                                "Received message on client_name = {} topic = {}: message = {:?}",
                                                                client_name_clone, topic, payload_str
                                                            );

                                                            // 构建 MQTTMessage 并发布到 RabbitMQ
                                                            let mqtt_msg = MQTTMessage {
                                                                mqtt_client_id: client_name_clone.clone(),
                                                                message: payload_str.to_string(),
                                                            };

                                                     let rabbitmq_instance = match get_rabbitmq_instance().await {
                    Ok(instance) => instance,
                    Err(e) => {
                        error!("Failed to get RabbitMQ instance: {:?}", e);
                  continue;
                    }
                };
                let mut rabbitmq = rabbitmq_instance.lock().await;

                // 创建 channel
                let channel = match rabbitmq.connection.create_channel().await {
                    Ok(ch) => ch,
                    Err(e) => {
                        error!("Failed to create channel: {:?}", e);
                             continue;

                    }
                };


                                                            if let Err(e) = channel
                                                                .basic_publish(
                                                                    "",
                                                                    "pre_handler",
                                                                    BasicPublishOptions::default(),
                                                                    mqtt_msg.to_json_string().as_bytes(),
                                                                    BasicProperties::default(),
                                                                )
                                                                .await
                                                            {
                                                                error!("Failed to publish message: {:?}", e);
                                                            }
                                                        }
                                                        Incoming::ConnAck(_) => {
                                                            info!("Connected to broker");
                                                        }
                                                        Incoming::SubAck(_) => {
                                                            info!("Subscription acknowledged");
                                                        }
                                                        _ => {
                                                            info!("Other incoming event: {:?}", incoming);
                                                        }
                                                    }
                                                }
                                                Event::Outgoing(outgoing) => {
                                                    match outgoing {
                                                        Outgoing::PingReq => {
                                                            info!("Ping request sent");
                                                        }
                                                        Outgoing::PingResp => {
                                                            info!("Ping response received");
                                                        }
                                                        _ => {
                                                            info!("Other outgoing event: {:?}", outgoing);
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    info!("Other event: {:?}", event);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!("Error: {:?}", e);
                                            break;
                                        }
                                    }
                                }
                            }
            }
        }
    });

    let mut clients = CLIENT_MANAGER.lock();
    clients?.insert(
        client_name.clone(),
        MqttClientState {
            client: client.clone(),
            shutdown_tx,
            eventloop_handle,
        },
    );

    Ok((client,))
}

use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel, Connection};

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
            debug!("Ping Response received");
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
            debug!("Ping Response received");
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
