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

async fn create_client(
    client_name: &str,
    topic: &str,
) -> Result<(AsyncClient, rumqttc::EventLoop), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new(client_name, "localhost", 1883);
    mqttoptions.set_credentials("admin", "admin");

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

fn handler_event(
    event: &Result<Event, ConnectionError>,
    topic: &str,
) -> Option<Result<(), Box<dyn Error>>> {
    match event {
        Ok(Event::Incoming(Incoming::Publish(publish))) => {
            let payload_str =
                std::str::from_utf8(&publish.payload).unwrap_or_else(|_| "<Invalid UTF-8>");
            info!(
                "Received message on topic = {}: message = {:?}",
                topic, payload_str
            );
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
            println!("Ping Response received");
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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();
    init_mqtt_map()?;

    let (client1, mut eventloop1) = create_client("test-1", "/tt1").await?;
    let (client2, mut eventloop2) = create_client("test-2", "/tt2").await?;

    let client_name_clone = "test-1".to_string();
    tokio::spawn(async move {
        time::sleep(Duration::from_secs(10)).await;
        disconnect_client(client_name_clone).await;
    });

    loop {
        tokio::select! {
            event = eventloop1.poll() => {
                if let Err(e) = event {
                    error!("Error polling eventloop 1: {:?}", e);
                } else {
                    handler_event(&event, "/tt1");
                }
            }
            event2 = eventloop2.poll() => {
                if let Err(e) = event2 {
                    error!("Error polling eventloop 2: {:?}", e);
                } else {
                    handler_event(&event2, "/tt2");
                }
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
