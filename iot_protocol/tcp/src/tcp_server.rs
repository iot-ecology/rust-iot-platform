use chrono::Utc;
use common_lib::models::{Auth, TcpMessage};
use common_lib::rabbit_utils::get_rabbitmq_instance;
use common_lib::redis_handler::RedisWrapper;
use log::{error, info};
use once_cell::sync::Lazy;
use serde_json::from_str;
use std::collections::HashMap;
use std::sync::Arc;
use time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub static CLIENTS: Lazy<Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub(crate) struct TcpServer {
    address: String,
    redis_wrapper: RedisWrapper,
    name: String,
    size: u8,
}

impl TcpServer {
    pub(crate) fn new(address: &str, redis_wrapper: RedisWrapper, name: String, size: u8) -> Self {
        TcpServer {
            address: address.to_string(),
            redis_wrapper,
            name,
            size,
        }
    }

    pub(crate) async fn start(&self) {
        let listener = TcpListener::bind(&self.address)
            .await
            .expect("Failed to bind address");
        info!("Server is listening on {}", self.address);

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let handler = ConnectionHandler::new(
                        stream,
                        self.redis_wrapper.clone(),
                        self.name.clone(),
                        self.size,
                    );
                    tokio::spawn(async move {
                        handler.handle_connection().await;
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

struct ConnectionHandler {
    stream: Arc<Mutex<TcpStream>>,
    wrapper: RedisWrapper,
    name: String,
    size: u8,
}

impl ConnectionHandler {
    fn new(stream: TcpStream, wrapper: RedisWrapper, name: String, size: u8) -> Self {
        ConnectionHandler {
            stream: Arc::new(Mutex::new(stream)),
            wrapper,
            name,
            size,
        }
    }

    async fn handle_connection(mut self) {
        let peer_addr = self
            .stream
            .lock()
            .await
            .peer_addr()
            .expect("Failed to get peer address");
        let peer_addr_str = peer_addr.to_string();
        info!("Client connected: {}", peer_addr_str);

        let mut buffer = [0; 512];

        loop {
            let mut stream = self.stream.lock().await;

            match stream.read(&mut buffer).await {
                Ok(bytes_read) if bytes_read == 0 => {
                    info!("Client disconnected: {}", peer_addr);
                    drop(stream);
                    self.cleanup_connection(peer_addr_str.clone()).await;
                    break;
                }
                Ok(bytes_read) => {
                    let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    info!("Received from {}, data= {}", peer_addr, received_message);
                    drop(stream);
                    if let Err(e) = self
                        .send_response(&received_message, peer_addr_str.clone())
                        .await
                    {
                        error!("Failed to send response: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to read from connection: {}", e);
                    break;
                }
            }
        }
    }

    async fn send_response(
        &mut self,
        message: &str,
        remote_address: String,
    ) -> tokio::io::Result<()> {
        let uid_option = self.get_uid(message, remote_address.clone()).await;

        if let Some(uid) = uid_option {
            info!("UID for {}: {}", remote_address, uid);
            self.process_message(message, uid, remote_address).await;
        } else if let Some(uid) = self.extract_uid(message) {
            if !self
                .validate_and_store_uid(message, uid, remote_address.clone())
                .await?
            {
                self.respond_with_message("账号密码不正确.\n").await?;
            }
        } else {
            self.respond_with_message("请发送uid:xxx格式的消息进行设备ID映射。\n")
                .await?;
        }
        Ok(())
    }

    async fn respond_with_message(&self, msg: &str) -> tokio::io::Result<()> {
        match self.stream.lock().await.write_all(msg.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to send message: {:?}", e);
                Err(e)
            }
        }
    }

    async fn validate_and_store_uid(
        &mut self,
        message: &str,
        uid: String,
        remote_address: String,
    ) -> tokio::io::Result<bool> {
        let device_info = self.parse_device_info(message).unwrap();
        let (username, password) = self.find_device_mapping_up(&device_info.0).await;
        if device_info.1 == username && device_info.2 == password {
            let ss = format!("tcp_uid_f:{}", self.name);
            if self
                .wrapper
                .get_hash_length(&ss)
                .await
                .unwrap()
                .unwrap_or(0)
                + 1
                <= self.size as usize
            {
                self.store_uid(&device_info.0, remote_address.clone()).await;
                CLIENTS
                    .lock()
                    .await
                    .insert(remote_address.clone(), self.stream.clone());
                self.respond_with_message("成功识别设备编码.\n").await?;
                return Ok(true);
            } else {
                self.respond_with_message("当前服务器已满载.\n").await?;
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn parse_device_info(&self, message: &str) -> Option<(String, String, String)> {
        let parts: Vec<&str> = message.trim().split(':').collect();
        if parts.len() == 4 && parts[0] == "uid" {
            let device_id = parts[1].to_string();
            let username = parts[2].to_string();
            let password = parts[3].to_string();
            Some((device_id, username, password))
        } else {
            None
        }
    }

    async fn store_uid(&mut self, device_id: &str, remote_address: String) {
        let k1 = format!("tcp_uid:{}", self.name);
        let k2 = format!("tcp_uid_f:{}", self.name);
        self.wrapper
            .set_hash(&k1, device_id, &remote_address)
            .await
            .expect("Failed to set hash");
        self.wrapper
            .set_hash(&k2, &remote_address, device_id)
            .await
            .expect("Failed to set hash");
    }

    async fn cleanup_connection(&mut self, remote_address: String) {
        let k1 = format!("tcp_uid:{}", self.name);
        let k2 = format!("tcp_uid_f:{}", self.name);
        if let Some(device_id) = self.wrapper.get_hash(&k2, &remote_address).await.unwrap() {
            self.wrapper
                .delete_hash_field(&k1, &device_id)
                .await
                .expect("Failed to delete hash field");
            self.wrapper
                .delete_hash_field(&k2, &remote_address)
                .await
                .expect("Failed to delete hash field");
        }
    }

    async fn find_device_mapping_up(&mut self, device_id: &str) -> (String, String) {
        if let Some(string) = self.wrapper.get_hash("auth:tcp", device_id).await.unwrap() {
            let auth: Auth = from_str(&string).expect("Failed to deserialize Auth");
            (auth.username, auth.password)
        } else {
            (String::new(), String::new())
        }
    }

    fn extract_uid(&self, message: &str) -> Option<String> {
        if message.starts_with("uid:") {
            Some(message[4..].trim().to_string())
        } else {
            None
        }
    }

    async fn process_message(&mut self, message: &str, uid: String, remote_address: String) {
        info!("Processing message: {}", message);

        let string1 = remote_address.replace(":", "@");
        let now = Utc::now().timestamp();
        let key = format!("tcp:last:{}", string1);
        self.wrapper
            .set_string_with_expiry(&key, &now.to_string(), 24 * 60 * 60)
            .await
            .unwrap();

        let tcp = TcpMessage {
            uid: uid.clone(),
            message: message.replace("\n", ""),
        };
        let json_string = serde_json::to_string(&tcp).unwrap();
        info!("Send MQ : {}", tcp.to_json_string());
        let k1 = format!("tcp_uid:{}", self.name);
        let option = self
            .wrapper
            .get_hash(k1.as_str(), uid.as_str())
            .await
            .expect("Failed to get hash");
        if option.is_some() {
            let rabbit = get_rabbitmq_instance().await.unwrap();

            rabbit
                .publish("", "pre_tcp_handler", json_string.as_str())
                .await
                .expect("publish message failed");

            self.respond_with_message("数据已处理.\n")
                .await
                .expect("process_message error");
        }
    }

    async fn get_uid(&self, message: &str, remote_address: String) -> Option<String> {
        let key = format!("tcp_uid_f:{}", self.name);
        self.wrapper.get_hash(&key, &remote_address).await.unwrap()
    }
}
