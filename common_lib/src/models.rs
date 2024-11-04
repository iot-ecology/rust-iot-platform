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
#[derive(Debug)]
pub enum DataValue {
    Float(f64),
    Text(String),
    Integer(i64),
}
#[derive(Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub cache_size: u64,
    #[serde(rename = "ID")] // 在序列化时使用 "ID"
    pub id: i64,
    pub r#type: String,
    unit: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignalWaringConfig {
    #[serde(rename = "signal_id")]
    pub signal_id: i32, // 信号表的外键ID
    #[serde(rename = "min")]
    pub min: f64, // 范围, 小值
    #[serde(rename = "max")]
    pub max: f64, // 范围, 大值
    #[serde(rename = "in_or_out")]
    pub in_or_out: i32, // 1 范围内报警 0 范围外报警
    #[serde(rename = "unit")]
    pub unit: Option<String>, // 单位
    #[serde(rename = "ID")]
    pub id: i32, // ID
}

#[derive(Debug)]
pub struct SignalMapping {
    pub cache_size: u64,
    pub id: i64,
    pub numb: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDelayWaringParam {
    #[serde(rename = "mqtt_client_name")]
    pub mqtt_client_name: String, // MQTT客户端的名称，不存储在数据库中

    pub protocol: String,

    pub identification_code: String, // 设备标识码

    pub device_uid: i32, // MQTT客户端表的外键ID

    pub name: String, // 参数名称
    #[serde(rename = "signal_name")]
    pub signal_name: String, // 信号表 name
    #[serde(rename = "signal_id")]
    pub signal_id: i32, // 信号表的外键ID
    pub signal_delay_waring_id: i32, // SignalDelayWaring 主键
    #[serde(rename = "ID")]
    pub id: i32, // ID
}
#[derive(Debug, serde::Deserialize)]
pub struct SignalDelayWaring {
    pub name: String,
    pub script: String,
    #[serde(rename = "ID")]
    pub id: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Tv {
    pub time: i64,
    pub value: f64,
}
