use common_lib::init_logger;
use log::{debug, error, info};
use once_cell::sync::OnceCell;
use rumqttc::{AsyncClient, ConnectionError, Event, Incoming, MqttOptions, QoS};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{task, time};

static CLIENTS: OnceCell<Arc<Mutex<HashMap<String, AsyncClient>>>> = OnceCell::new();

fn init_mqtt_map() -> Result<(), Box<dyn Error>> {
    let clients = Arc::new(Mutex::new(HashMap::new()));
    CLIENTS.set(clients).unwrap();
    Ok(())
}
fn get_client(client_name: &str) -> Option<AsyncClient> {
    let clients_lock = CLIENTS.get().unwrap().lock().unwrap();
    clients_lock.get(client_name).cloned()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();
    init_mqtt_map()?;

    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    mqttoptions.set_credentials("admin", "admin");
    let client_name = "test-1".to_string();
    let (client, mut eventloop) = AsyncClient::new(mqttoptions.clone(), 10);

    {
        let mut clients_lock = CLIENTS.get().unwrap().lock().unwrap();
        client.subscribe("/tt1", QoS::AtMostOnce).await.unwrap();
        clients_lock.insert(client_name.clone(), client);
    }

    let mut mqttoptions2 = MqttOptions::new("test-2", "localhost", 1883);
    mqttoptions2.set_credentials("admin", "admin");
    let client_name2 = "test-2".to_string();
    let (client2, mut eventloop2) = AsyncClient::new(mqttoptions2.clone(), 10);

    {
        let mut clients_lock = CLIENTS.get().unwrap().lock().unwrap();
        client2.subscribe("/tt2", QoS::AtMostOnce).await.unwrap();
        clients_lock.insert(client_name2.clone(), client2);
    }

    let client_name_clone = client_name.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(10)).await;
        if let Some(client) = {
            let mut clients_lock = CLIENTS.get().unwrap().lock().unwrap();
            clients_lock.remove(&client_name_clone)
        } {
            client.disconnect().await.unwrap();
            info!("Disconnected client: {}", client_name_clone);
        }
    });

    loop {
        tokio::select! {
            event = eventloop.poll() => {
                match event {
                    Ok(event) => {
                        if let Some(value) = handler_event(&Ok(event), "/test/1") {
                            return value;
                        }
                    }
                    Err(e) => {
                        error!("Error polling eventloop 1: {:?}", e);
                        continue; // 这里可以选择继续或退出，取决于你的需求
                    }
                }
            }
            event2 = eventloop2.poll() => {
                match event2 {
                    Ok(event) => {
                        if let Some(value) = handler_event(&Ok(event), "/test/2") {
                            return value;
                        }
                    }
                    Err(e) => {
                        error!("Error polling eventloop 2: {:?}", e);
                        continue; // 同样可以选择继续或退出
                    }
                }
            }
        }
    }
}

fn handler_event(
    event: &Result<Event, ConnectionError>,
    x: &str,
) -> Option<Result<(), Box<dyn Error>>> {
    match &event {
        Ok(Event::Incoming(Incoming::Publish(publish))) => {
            let topic = &publish.topic;
            let payload = &publish.payload;

            let payload_str =
                std::str::from_utf8(&publish.payload).unwrap_or_else(|_| "<Invalid UTF-8>");

            info!(
                "Received message on topic =  {}: message = {:?}",
                topic, payload_str
            );
        }
        Ok(Event::Incoming(Incoming::ConnAck(connack))) => {
            debug!("Connection Acknowledged: {:?}", connack);
        }
        Ok(Event::Incoming(Incoming::PubAck(puback))) => {
            println!("Publish Acknowledged: pkid={}", puback.pkid);
        }
        Ok(Event::Incoming(Incoming::PubRec(pubrec))) => {
            debug!("Publish Received: pkid={}", pubrec.pkid);
        }
        Ok(Event::Incoming(Incoming::PubRel(pubrel))) => {
            println!("Publish Released: pkid={}", pubrel.pkid);
        }
        Ok(Event::Incoming(Incoming::PubComp(pubcomp))) => {
            debug!("Publish Completed: pkid={}", pubcomp.pkid);
        }
        Ok(Event::Incoming(Incoming::SubAck(suback))) => {
            info!(
                "Subscribe Acknowledged: pkid={}, return_codes={:?}",
                suback.pkid, suback.return_codes
            );
        }
        Ok(Event::Incoming(Incoming::UnsubAck(unsuback))) => {
            println!("Unsubscribe Acknowledged: pkid={}", unsuback.pkid);
        }
        Ok(Event::Incoming(Incoming::PingResp)) => {
            println!("Ping Response received");
        }
        Ok(v) => {
            debug!("Other Event = {v:?}");
        }
        Err(e) => {
            error!("Error = {e:?}");
            return Some(Ok(()));
        }
    }
    None
}

async fn requests(client: AsyncClient) {
    client
        .subscribe("hello/world", QoS::AtMostOnce)
        .await
        .unwrap();
    for i in 1..=10 {
        client
            .publish("hello/world", QoS::ExactlyOnce, false, "hello")
            .await
            .unwrap();
        time::sleep(Duration::from_secs(1)).await;
    }
    time::sleep(Duration::from_secs(120)).await;
}
