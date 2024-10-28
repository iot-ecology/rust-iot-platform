use std::collections::HashMap;
use common_lib::models::{Auth, TcpMessage};
use common_lib::redis_handler::RedisWrapper;
use serde_json::from_str;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use tokio::net::unix::uid_t;

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

    pub(crate) fn start(&self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind address");
        println!("Server is listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let handler = ConnectionHandler::new(stream, self.redis_wrapper.clone(), self.name.clone(), self.size);

                    thread::spawn(move || {
                        tokio::runtime::Runtime::new().unwrap().block_on(async {
                            handler.handle_connection().await;
                        });
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

struct ConnectionHandler {
    stream: TcpStream,
    wrapper: RedisWrapper,
    name: String,
    size: u8,
}

impl ConnectionHandler {
    fn new(stream: TcpStream, wrapper: RedisWrapper, name: String, size: u8) -> Self {
        ConnectionHandler { stream, wrapper, name, size }
    }

    async fn handle_connection(mut self) {
        let peer_addr = self.stream.peer_addr().expect("Failed to get peer address");
        let peer_addr_str = peer_addr.to_string();
        println!("Client connected: {}", peer_addr_str);

        let mut buffer = [0; 512];

        loop {
            match self.stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        println!("Client disconnected: {}", peer_addr);
                        break;
                    }
                    let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Received from {}: {}", peer_addr, received_message);

                    // 处理消息并发送响应
                    if let Err(e) = self.send_response(&received_message, peer_addr_str.clone()).await {
                        eprintln!("Failed to send response: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from connection: {}", e);
                    break;
                }
            }
        }
    }

    async fn send_response(&mut self, message: &str, remote_address: String) -> std::io::Result<()> {
        let option = self.get_uid(message, remote_address.clone()).await;

        if let Some(uid) = option {
            println!("UID for {}: {}", remote_address, uid);
            // 使用 UID 进行进一步处理
            self.handler_message(message, uid);
        } else {
            let uid = self.extract_uid(message);
            if uid.is_none() {
                self.stream.write_all("请发送uid:xxx格式的消息进行设备ID映射。\n".as_bytes())?;


                return Ok(());
            } else {
                let string = message.replace("\n", "");
                let parts: Vec<&str> = string.split(':').collect();
                if parts.len() != 4 {
                    self.stream.write_all("uid格式错误.\n".as_bytes())?;
                } else {
                    let device_id = parts[1];
                    let username = parts[2];
                    let password = parts[3];
                    // 这里可以处理 device_id、username 和 password

                    let x = self.find_device_mapping_up(device_id).await;
                    println!("username {} password {}", x.0, x.1);

                    if x.0.as_str() == username && x.1.as_str() == password {

                        //     获取当前服务名称上面 tcp 客户端的总数量

                        let ss = format!("tcp_uid_f:{}", self.name);
                        let option1 = self.wrapper.get_hash_length(ss.as_str()).await.unwrap().unwrap_or(0);

                        if option1 +1 <= self.size as usize {
                            self.storage_uid(device_id, remote_address).await;

                            self.stream.write_all("成功识别设备编码.\n".as_bytes())?;
                            return Ok(());
                        } else {
                            self.stream.write_all("当前服务器已满载.\n".as_bytes())?;
                            return Ok(());
                        }
                    } else {
                        self.stream.write_all("账号密码不正确".as_bytes())?;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    async fn storage_uid(&mut self, device_id: &str, remote_address: String) {
        let k1 = format!("tcp_uid:{}", self.name);
        let k2 = format!("tcp_uid_f:{}", self.name);

        self.wrapper.set_hash(k1.as_str(), device_id, remote_address.as_str()).await.expect("TODO: panic message");
        self.wrapper.set_hash(k2.as_str(), remote_address.as_str(), device_id).await.expect("TODO: panic message");
    }

    async fn find_device_mapping_up(&mut self, device_id: &str) -> (String, String) {
        let option = self.wrapper.get_hash("auth:tcp", device_id).await.unwrap();

        if option.is_none() {
            return (String::new(), String::new());
        } else {
            let string = option.unwrap();

            // 将 string 解析为 Auth 结构体
            let auth: Auth = from_str(&string).expect("Failed to deserialize Auth");
            return (auth.username, auth.password);
        }
    }
    fn extract_uid(&self, message: &str) -> Option<String> {
        if message.starts_with("uid:") {
            let uid = &message[4..].trim(); // 提取并去除前导空格
            return Some(uid.to_string());
        }
        None
    }
    fn handler_message(&mut self, message: &str, remote_address: String) {
        // self.stream.write_all(b"No UID found");
        // todo: 写出到消息队列
        println!("Handler message: {}", message);

        let string = message.replace("\n", "");
        let tcp = TcpMessage {
            uid: remote_address
            ,
            message: string,
        };

        println!("Send MQ : {}", tcp.to_json_string());
        self.stream.write_all("数据已处理.\n".as_bytes()).expect("handler_message error");


    }

    // 从 redis 中获取
    async fn get_uid(&self, message: &str, remote_address: String) -> Option<String> {
        let key = format!("tcp_uid_f:{}", self.name);
        println!("key = {}  remote_address = {}", key, remote_address);
        if let Ok(option) = self.wrapper.get_hash(key.as_str(), remote_address.as_str()).await {
            return option;
        }
        None
    }

    fn close_connection(&mut self) {
        if let Err(e) = self.stream.shutdown(std::net::Shutdown::Both) {
            eprintln!("Failed to shutdown connection: {}", e);
        } else {
            println!("Connection closed");
        }
    }
}


