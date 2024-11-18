extern crate rocket;

use crate::api::ctr::AppState;
use common_lib::config::{get_config, read_config};
use common_lib::models::{TcpMessage, WsMessage};
use common_lib::rabbit_utils::{get_rabbitmq_instance, init_rabbitmq_with_config};
use common_lib::redis_handler::init_redis;
use log::info;
use rocket::futures::channel::mpsc::{channel, Sender};
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio::select;
use rocket::tokio::sync::Mutex;
use rocket::tokio::time::{self, timeout};
use rocket::{get, launch, routes, State};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use ws::{Channel, Message, WebSocket};

pub mod api;

type PeersMap = Arc<Mutex<HashMap<String, Sender<String>>>>;

#[get("/ws?<id>")]
fn mirror(id: String, ws: WebSocket, peers_map: &State<PeersMap>) -> Channel<'static> {
    let peers_map = peers_map.inner().clone();
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let (tx, mut rx) = channel(1);
            peers_map.lock().await.insert(id.clone(), tx);
            let count = peers_map.lock().await.len();
            info!("Connection opened ({} clients)", count);

            loop {
                select! {
                            message = stream.next() => match message {
                                Some(Ok(Message::Text(text))) => {
                                    info!("Received message: {:?}", text.clone());

  let parts: Vec<&str> = id.split('@').collect();
                                    let uid = parts.get(0).expect("UID is missing"); // 抛出异常

                                   //  id 按照字符串@拆分取第一个元素
                                   let c =  WsMessage{
                                        uid:uid.to_string(),
                                        message:text
                                    };

       let rabbit = get_rabbitmq_instance().await.unwrap();
            let guard = rabbit.lock().await;

            guard
                .publish("", "pre_ws_handler", c.to_json_string().as_str())
                .await
                .expect("publish message failed");


                                    let _ = stream.send(Message::Text(format!("消息已处理"))).await;
                                    peers_map.lock().await.iter().for_each(|(peer, tx)| {
                                        if peer != &id {
                                            let _ = tx.clone().send(format!("{} ", id));
                                        }
                                    });
                                }
                                Some(Ok(message)) => {
                                    info!("Received message from client: {:?}", message);
                                    let _ = stream.send(message).await;
                                }
                                Some(Err(error)) => {
                                    info!("Error: {:?}", error);
                                    break;
                                }
                                None => break,
                            },
                            Some(message) = rx.next() => {
                                info!("Received message from other client: {:?}", message);
                                let _ = stream.send(Message::Text(message)).await;
                            },
                            else => break,
                        }
            }

            peers_map.lock().await.remove(&id);
            let count = peers_map.lock().await.len();
            info!("Connection closed ({} clients)", count);

            Ok(())
        })
    })
}

#[launch]
fn rocket() -> _ {
    let peers_map: PeersMap = Arc::new(Mutex::new(HashMap::new()));
    let device_map = Arc::new(AppState {
        device_map: Mutex::new(HashMap::new()), // 这里可以填充数据
        max_connections: 10,                    // 设置最大连接数
        current_connections: Mutex::new(0),
    });

    init_logger(); // 初始化日志记录
    let rt = Runtime::new().unwrap();

    // 读取配置并初始化 Redis 和 RabbitMQ
    let config = rt.block_on(read_config("app-local.yml")).unwrap(); // 读取配置
    let guard1 = rt.block_on(get_config()).unwrap(); // 获取配置
    rt.block_on(init_redis(guard1.redis_config.clone()))
        .unwrap(); // 初始化 Redis
    rt.block_on(init_rabbitmq_with_config(guard1.mq_config.clone()))
        .unwrap(); // 初始化 RabbitMQ

    // 构建并启动 Rocket 应用
    rocket::build()
        .configure(rocket::Config {
            port: guard1.node_info.port,
            ..Default::default()
        })
        .manage(peers_map)
        .manage(device_map)
        .mount("/", routes![mirror, api::ctr::auth_api])
}

fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}
