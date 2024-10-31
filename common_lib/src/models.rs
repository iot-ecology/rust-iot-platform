use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
pub struct DataRowList {
    pub time: i64,                   // 秒级时间戳
    pub device_uid: String,          // 能够产生网络通讯的唯一编码
    pub identification_code: String, // 设备标识码
    pub data: Vec<DataRow>,          // 数据行
    pub nc: String,                  // Nc 字段
    #[serde(skip_serializing_if = "Option::is_none")] // 如果为 None，则不序列化
    pub protocol: Option<String>, // 协议字段
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataRow {
    pub name: String,  // 数据行名称
    pub value: String, // 数据行值
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TcpMessage {
    pub uid: String,
    pub message: String,
}

impl TcpMessage {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize TcpMessage")
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HttpMessage {
    pub uid: String,
    pub message: String,
}

impl HttpMessage {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize HttpMessage")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsMessage {
    pub uid: String,
    pub message: String,
}

impl WsMessage {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize WsMessage")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoapMessage {
    pub uid: String,
    pub message: String,
}

impl CoapMessage {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize CoapMessage")
    }
}
