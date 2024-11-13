use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, Debug)]

pub struct User {
    pub id: u64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
impl FromRow<'_, sqlx::mysql::MySqlRow> for User {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            email: row.try_get("email")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct CalcParam {
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub device_uid: Option<i64>,
    pub name: Option<String>,
    pub signal_name: Option<String>,
    pub signal_id: Option<i64>,
    pub reduce: Option<String>,
    pub calc_rule_id: Option<i64>,
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct CalcRule {
    pub name: Option<String>,
    pub cron: Option<String>,
    pub script: Option<String>,
    pub offset: Option<i64>,
    pub start: Option<bool>,
    pub mock_value: Option<String>,
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct CassandraTransmitBind {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_uid: Option<i64>,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub cassandra_transmit_id: Option<i32>,
    pub database: Option<String>,
    pub table_name: Option<String>,
    pub script: Option<String>,
    pub enable: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct CassandraTransmit {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct ClickhouseTransmitBind {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_uid: Option<i64>,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub clickhouse_transmit_id: Option<i32>,
    pub database: Option<String>,
    pub script: Option<String>,
    pub table_name: Option<String>,
    pub enable: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct ClickhouseTransmit {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct CoapHandler {
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Dashboard {
    pub name: Option<String>,
    pub config: Option<String>,
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Dept {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub parent_id: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct DeviceBindMqttClient {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_info_id: Option<u64>,
    pub mqtt_client_id: Option<u64>,
    pub identification_code: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct DeviceBindTcpHandler {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_info_id: Option<u64>,
    pub tcp_handler_id: Option<u64>,
    pub identification_code: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct DeviceGroupBindMqttClient {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_group_id: Option<u64>,
    pub mqtt_client_id: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct DeviceGroupDevice {
    pub device_info_id: Option<u64>,
    pub device_group_id: Option<u64>,
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct DeviceGroup {
    pub name: Option<String>,
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct DeviceInfo {
    pub product_id: Option<u64>,
    pub sn: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub manufacturing_date: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub procurement_date: Option<chrono::NaiveDateTime>,
    pub source: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub warranty_expiry: Option<chrono::NaiveDateTime>,
    pub push_interval: Option<i64>,
    pub error_rate: Option<f64>,
    pub protocol: Option<String>,
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub identification_code: Option<String>,
    pub device_uid: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct DingDing {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub access_token: Option<String>,
    pub secret: Option<String>,
    pub content: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct FeiShu {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub access_token: Option<String>,
    pub secret: Option<String>,
    pub content: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct HttpHandler {
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct InfluxDbTransmitBind {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_uid: Option<i64>,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub influxdb_transmit_id: Option<i32>,
    pub bucket: Option<String>,
    pub org: Option<String>,
    pub measurement: Option<String>,
    pub script: Option<String>,
    pub enable: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct InfluxDbTransmit {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub token: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct MessageList {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub content: Option<String>,
    pub en_content: Option<String>,
    pub message_type_id: Option<i64>,
    pub ref_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct MessageTypeBindRole {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub message_type: Option<i64>,
    pub role_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct MongoTransmitBind {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_uid: Option<i64>,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub mysql_transmit_id: Option<i32>,
    pub collection: Option<String>,
    pub database: Option<String>,
    pub script: Option<String>,
    pub enable: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct MongoTransmit {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub port: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct MqttClient {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub host: Option<String>,
    pub port: Option<i64>,
    pub client_id: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub subtopic: Option<String>,
    pub start: Option<bool>,
    pub script: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct MysqlTransmitBind {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub protocol: Option<String>,
    pub device_uid: Option<String>,
    pub identification_code: Option<String>,
    pub mysql_transmit_id: Option<i32>,
    pub table_name: Option<String>,
    pub script: Option<String>,
    pub enable: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct MysqlTransmit {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct ProductPlan {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub production_plan_id: Option<u64>,
    pub product_id: Option<u64>,
    pub quantity: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct ProductionPlan {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub start_date: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub end_date: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub status: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Product {
    pub id: u64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub cost: Option<f64>,
    pub quantity: Option<i64>,
    pub minimum_stock: Option<i64>,
    pub warranty_period: Option<i64>,
    pub status: Option<String>,
    pub tags: Option<String>,
    pub image_url: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct RepairRecord {
    pub id: u64,
    pub device_group_group_id: Option<u64>,
    pub device_info_id: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub repair_date: Option<chrono::NaiveDateTime>,
    pub technician: Option<String>,
    pub cost: Option<f64>,
    pub description: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Role {
    pub id: u64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub can_del: Option<bool>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct ShipmentProductDetail {
    pub id: u64,
    pub shipment_record_id: Option<u64>,
    pub product_id: Option<u64>,
    pub device_info_id: Option<u64>,
    pub quantity: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct ShipmentRecord {
    pub id: u64,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub shipment_date: Option<chrono::NaiveDateTime>,
    pub technician: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_address: Option<String>,
    pub tracking_number: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct SignalDelayWaringParam {
    pub id: u64,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub device_uid: Option<u64>,
    pub name: Option<String>,
    pub signal_name: Option<String>,
    pub signal_id: Option<u64>,
    pub signal_delay_waring_id: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct SignalDelayWaring {
    pub id: u64,
    pub name: Option<String>,
    pub script: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct SignalWaringConfig {
    pub id: u64,
    pub signal_id: Option<u64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub in_or_out: Option<u64>,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub device_uid: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Signal {
    pub id: u64,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub device_uid: Option<u64>,
    pub name: Option<String>,
    pub alias: Option<String>,
    pub signal_type: Option<String>,
    pub unit: Option<String>,
    pub cache_size: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct SimCard {
    pub id: u64,
    pub access_number: String,
    pub iccid: String,
    pub imsi: String,
    pub operator: String,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub expiration: chrono::NaiveDateTime,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct SimUseHistory {
    pub id: u64,
    pub sim_id: Option<u64>,
    pub device_info_id: Option<u64>,
    pub description: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct TcpHandler {
    pub id: u64,
    pub device_info_id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub script: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct UserBindDeviceInfo {
    pub id: u64,
    pub user_id: Option<u64>,
    pub device_id: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct UserDept {
    pub id: u64,
    pub user_id: Option<u64>,
    pub dept_id: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct UserRole {
    pub id: u64,
    pub user_id: Option<u64>,
    pub role_id: Option<u64>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct WebSocketHandler {
    pub id: u64,
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

fn serialize_naive_datetime<S>(
    naive: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match naive {
        Some(ndt) => {
            // 将 NaiveDateTime 转为 DateTime<Utc> 并序列化
            let dt = DateTime::<Utc>::from_utc(*ndt, Utc);
            dt.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}

fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let dt: Option<DateTime<Utc>> = Option::deserialize(deserializer)?;
    Ok(dt.map(|d| d.naive_utc()))
}
