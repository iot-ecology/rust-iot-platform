use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
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
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CalcRule {
    pub name: Option<String>,
    pub cron: Option<String>,
    pub script: Option<String>,
    pub offset: Option<i64>,
    pub start: Option<bool>,
    pub mock_value: Option<String>,
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct CassandraTransmitBind {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
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
#[derive(Serialize, Deserialize)]
pub struct CassandraTransmit {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ClickhouseTransmitBind {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
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
#[derive(Serialize, Deserialize)]
pub struct ClickhouseTransmit {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct CoapHandler {
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Dashboard {
    pub name: Option<String>,
    pub config: Option<String>,
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct Dept {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub parent_id: Option<u64>,
}
#[derive(Serialize, Deserialize)]
pub struct DeviceBindMqttClient {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_info_id: Option<u64>,
    pub mqtt_client_id: Option<u64>,
    pub identification_code: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct DeviceBindTcpHandler {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_info_id: Option<u64>,
    pub tcp_handler_id: Option<u64>,
    pub identification_code: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct DeviceGroupBindMqttClient {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub device_group_id: Option<u64>,
    pub mqtt_client_id: Option<u64>,
}
#[derive(Serialize, Deserialize)]
pub struct DeviceGroupDevice {
    pub device_info_id: Option<u64>,
    pub device_group_id: Option<u64>,
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct DeviceGroup {
    pub name: Option<String>,
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    pub product_id: Option<u64>,
    pub sn: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub manufacturing_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub procurement_date: Option<chrono::NaiveDateTime>,
    pub source: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub warranty_expiry: Option<chrono::NaiveDateTime>,
    pub push_interval: Option<i64>,
    pub error_rate: Option<f64>,
    pub protocol: Option<String>,
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub identification_code: Option<String>,
    pub device_uid: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub struct DingDing {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub access_token: Option<String>,
    pub secret: Option<String>,
    pub content: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct FeiShu {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub access_token: Option<String>,
    pub secret: Option<String>,
    pub content: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct HttpHandler {
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct InfluxDbTransmitBind {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
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
#[derive(Serialize, Deserialize)]
pub struct InfluxDbTransmit {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub token: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct MessageList {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub content: Option<String>,
    pub en_content: Option<String>,
    pub message_type_id: Option<i64>,
    pub ref_id: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct MessageTypeBindRole {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub message_type: Option<i64>,
    pub role_id: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct MongoTransmitBind {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
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
#[derive(Serialize, Deserialize)]
pub struct MongoTransmit {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub port: Option<i32>,
}
#[derive(Serialize, Deserialize)]
pub struct MqttClient {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
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
#[derive(Serialize, Deserialize)]
pub struct MysqlTransmitBind {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub protocol: Option<String>,
    pub device_uid: Option<String>,
    pub identification_code: Option<String>,
    pub mysql_transmit_id: Option<i32>,
    pub table_name: Option<String>,
    pub script: Option<String>,
    pub enable: Option<bool>,
}
#[derive(Serialize, Deserialize)]
pub struct MysqlTransmit {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ProductPlan {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub production_plan_id: Option<u64>,
    pub product_id: Option<u64>,
    pub quantity: Option<u64>,
}
#[derive(Serialize, Deserialize)]
pub struct ProductionPlan {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub end_date: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub status: Option<String>,
}
#[derive(Serialize, Deserialize)]
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
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct RepairRecord {
    pub id: u64,
    pub device_group_group_id: Option<u64>,
    pub device_info_id: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub repair_date: Option<chrono::NaiveDateTime>,
    pub technician: Option<String>,
    pub cost: Option<f64>,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct Role {
    pub id: u64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub can_del: Option<bool>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct ShipmentProductDetail {
    pub id: u64,
    pub shipment_record_id: Option<u64>,
    pub product_id: Option<u64>,
    pub device_info_id: Option<u64>,
    pub quantity: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct ShipmentRecord {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub shipment_date: Option<chrono::NaiveDateTime>,
    pub technician: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_address: Option<String>,
    pub tracking_number: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct SignalDelayWaringParam {
    pub id: u64,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub device_uid: Option<u64>,
    pub name: Option<String>,
    pub signal_name: Option<String>,
    pub signal_id: Option<u64>,
    pub signal_delay_waring_id: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct SignalDelayWaring {
    pub id: u64,
    pub name: Option<String>,
    pub script: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct SignalWaringConfig {
    pub id: u64,
    pub signal_id: Option<u64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub in_or_out: Option<u64>,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub device_uid: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
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
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct SimCard {
    pub id: u64,
    pub access_number: String,
    pub iccid: String,
    pub imsi: String,
    pub operator: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expiration: chrono::NaiveDateTime,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct SimUseHistory {
    pub id: u64,
    pub sim_id: Option<u64>,
    pub device_info_id: Option<u64>,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct TcpHandler {
    pub id: u64,
    pub device_info_id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub script: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct UserBindDeviceInfo {
    pub id: u64,
    pub user_id: Option<u64>,
    pub device_id: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDept {
    pub id: u64,
    pub user_id: Option<u64>,
    pub dept_id: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct UserRole {
    pub id: u64,
    pub user_id: Option<u64>,
    pub role_id: Option<u64>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Serialize, Deserialize)]
pub struct WebSocketHandler {
    pub id: u64,
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
