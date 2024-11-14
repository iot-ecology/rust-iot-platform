use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "ID")]
    pub id: u64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
    #[serde(
        rename = "UpdatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(
        rename = "DeletedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub deleted_at: Option<NaiveDateTime>,
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
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for CalcParam {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(CalcParam {
            protocol: row.try_get("protocol")?,
            identification_code: row.try_get("identification_code")?,
            device_uid: row.try_get("device_uid")?,
            name: row.try_get("name")?,
            signal_name: row.try_get("signal_name")?,
            signal_id: row.try_get("signal_id")?,
            reduce: row.try_get("reduce")?,
            calc_rule_id: row.try_get("calc_rule_id")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalcRule {
    pub name: Option<String>,
    pub cron: Option<String>,
    pub script: Option<String>,
    pub offset: Option<i64>,
    pub start: Option<bool>,
    pub mock_value: Option<String>,
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for CalcRule {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(CalcRule {
            name: row.try_get("name")?,
            cron: row.try_get("cron")?,
            script: row.try_get("script")?,
            offset: row.try_get("offset")?,
            start: row.try_get("start")?,
            mock_value: row.try_get("mock_value")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CassandraTransmitBind {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for CassandraTransmitBind {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(CassandraTransmitBind {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            device_uid: row.try_get("device_uid")?,
            protocol: row.try_get("protocol")?,
            identification_code: row.try_get("identification_code")?,
            cassandra_transmit_id: row.try_get("cassandra_transmit_id")?,
            database: row.try_get("database")?,
            table_name: row.try_get("table_name")?,
            script: row.try_get("script")?,
            enable: row.try_get("enable")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CassandraTransmit {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for CassandraTransmit {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(CassandraTransmit {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            host: row.try_get("host")?,
            port: row.try_get("port")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClickhouseTransmitBind {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for ClickhouseTransmitBind {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(ClickhouseTransmitBind {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            device_uid: row.try_get("device_uid")?,
            protocol: row.try_get("protocol")?,
            identification_code: row.try_get("identification_code")?,
            clickhouse_transmit_id: row.try_get("clickhouse_transmit_id")?,
            database: row.try_get("database")?,
            script: row.try_get("script")?,
            table_name: row.try_get("table_name")?,
            enable: row.try_get("enable")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClickhouseTransmit {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for ClickhouseTransmit {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(ClickhouseTransmit {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            host: row.try_get("host")?,
            port: row.try_get("port")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoapHandler {
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for CoapHandler {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(CoapHandler {
            device_info_id: row.try_get("device_info_id")?,
            name: row.try_get("name")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            script: row.try_get("script")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dashboard {
    pub name: Option<String>,
    pub config: Option<String>,
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for Dashboard {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Dashboard {
            name: row.try_get("name")?,
            config: row.try_get("config")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dept {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for Dept {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Dept {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            parent_id: row.try_get("parent_id")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceBindMqttClient {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for DeviceBindMqttClient {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DeviceBindMqttClient {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            device_info_id: row.try_get("device_info_id")?,
            mqtt_client_id: row.try_get("mqtt_client_id")?,
            identification_code: row.try_get("identification_code")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceBindTcpHandler {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for DeviceBindTcpHandler {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DeviceBindTcpHandler {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            device_info_id: row.try_get("device_info_id")?,
            tcp_handler_id: row.try_get("tcp_handler_id")?,
            identification_code: row.try_get("identification_code")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceGroupBindMqttClient {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for DeviceGroupBindMqttClient {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DeviceGroupBindMqttClient {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            device_group_id: row.try_get("device_group_id")?,
            mqtt_client_id: row.try_get("mqtt_client_id")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceGroupDevice {
    pub device_info_id: Option<u64>,
    pub device_group_id: Option<u64>,
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for DeviceGroupDevice {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DeviceGroupDevice {
            device_info_id: row.try_get("device_info_id")?,
            device_group_id: row.try_get("device_group_id")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceGroup {
    pub name: Option<String>,
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for DeviceGroup {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DeviceGroup {
            name: row.try_get("name")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
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
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for DeviceInfo {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DeviceInfo {
            product_id: row.try_get("product_id")?,
            sn: row.try_get("sn")?,
            manufacturing_date: row.try_get("manufacturing_date")?,
            procurement_date: row.try_get("procurement_date")?,
            source: row.try_get("source")?,
            warranty_expiry: row.try_get("warranty_expiry")?,
            push_interval: row.try_get("push_interval")?,
            error_rate: row.try_get("error_rate")?,
            protocol: row.try_get("protocol")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            identification_code: row.try_get("identification_code")?,
            device_uid: row.try_get("device_uid")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DingDing {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for DingDing {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DingDing {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            access_token: row.try_get("access_token")?,
            secret: row.try_get("secret")?,
            content: row.try_get("content")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeiShu {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for FeiShu {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(FeiShu {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            access_token: row.try_get("access_token")?,
            secret: row.try_get("secret")?,
            content: row.try_get("content")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpHandler {
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for HttpHandler {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(HttpHandler {
            device_info_id: row.try_get("device_info_id")?,
            name: row.try_get("name")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            script: row.try_get("script")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfluxDbTransmitBind {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for InfluxDbTransmitBind {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(InfluxDbTransmitBind {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            device_uid: row.try_get("device_uid")?,
            protocol: row.try_get("protocol")?,
            identification_code: row.try_get("identification_code")?,
            influxdb_transmit_id: row.try_get("influxdb_transmit_id")?,
            bucket: row.try_get("bucket")?,
            org: row.try_get("org")?,
            measurement: row.try_get("measurement")?,
            script: row.try_get("script")?,
            enable: row.try_get("enable")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfluxDbTransmit {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for InfluxDbTransmit {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(InfluxDbTransmit {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            host: row.try_get("host")?,
            port: row.try_get("port")?,
            token: row.try_get("token")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageList {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for MessageList {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(MessageList {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            content: row.try_get("content")?,
            en_content: row.try_get("en_content")?,
            message_type_id: row.try_get("message_type_id")?,
            ref_id: row.try_get("ref_id")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageTypeBindRole {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for MessageTypeBindRole {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(MessageTypeBindRole {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            message_type: row.try_get("message_type")?,
            role_id: row.try_get("role_id")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoTransmitBind {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for MongoTransmitBind {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(MongoTransmitBind {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            device_uid: row.try_get("device_uid")?,
            protocol: row.try_get("protocol")?,
            identification_code: row.try_get("identification_code")?,
            mysql_transmit_id: row.try_get("mysql_transmit_id")?,
            collection: row.try_get("collection")?,
            database: row.try_get("database")?,
            script: row.try_get("script")?,
            enable: row.try_get("enable")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct MongoTransmit {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for MongoTransmit {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(MongoTransmit {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            host: row.try_get("host")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            port: row.try_get("port")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MqttClient {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for MqttClient {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(MqttClient {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            host: row.try_get("host")?,
            port: row.try_get("port")?,
            client_id: row.try_get("client_id")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            subtopic: row.try_get("subtopic")?,
            start: row.try_get("start")?,
            script: row.try_get("script")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct MysqlTransmitBind {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for MysqlTransmitBind {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(MysqlTransmitBind {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            protocol: row.try_get("protocol")?,
            device_uid: row.try_get("device_uid")?,
            identification_code: row.try_get("identification_code")?,
            mysql_transmit_id: row.try_get("mysql_transmit_id")?,
            table_name: row.try_get("table_name")?,
            script: row.try_get("script")?,
            enable: row.try_get("enable")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct MysqlTransmit {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for MysqlTransmit {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(MysqlTransmit {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            host: row.try_get("host")?,
            port: row.try_get("port")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            database: row.try_get("database")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ProductPlan {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for ProductPlan {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(ProductPlan {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            production_plan_id: row.try_get("production_plan_id")?,
            product_id: row.try_get("product_id")?,
            quantity: row.try_get("quantity")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductionPlan {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for ProductionPlan {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(ProductionPlan {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            name: row.try_get("name")?,
            start_date: row.try_get("start_date")?,
            end_date: row.try_get("end_date")?,
            description: row.try_get("description")?,
            status: row.try_get("status")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Product {
    #[serde(rename = "ID")]
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
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for Product {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Product {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            sku: row.try_get("sku")?,
            price: row.try_get("price")?,
            cost: row.try_get("cost")?,
            quantity: row.try_get("quantity")?,
            minimum_stock: row.try_get("minimum_stock")?,
            warranty_period: row.try_get("warranty_period")?,
            status: row.try_get("status")?,
            tags: row.try_get("tags")?,
            image_url: row.try_get("image_url")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct RepairRecord {
    #[serde(rename = "ID")]
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
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for RepairRecord {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(RepairRecord {
            id: row.try_get("id")?,
            device_group_group_id: row.try_get("device_group_group_id")?,
            device_info_id: row.try_get("device_info_id")?,
            repair_date: row.try_get("repair_date")?,
            technician: row.try_get("technician")?,
            cost: row.try_get("cost")?,
            description: row.try_get("description")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Role {
    #[serde(rename = "ID")]
    pub id: u64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub can_del: Option<bool>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for Role {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Role {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            can_del: row.try_get("can_del")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ShipmentProductDetail {
    #[serde(rename = "ID")]
    pub id: u64,
    pub shipment_record_id: Option<u64>,
    pub product_id: Option<u64>,
    pub device_info_id: Option<u64>,
    pub quantity: Option<u64>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for ShipmentProductDetail {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(ShipmentProductDetail {
            id: row.try_get("id")?,
            shipment_record_id: row.try_get("shipment_record_id")?,
            product_id: row.try_get("product_id")?,
            device_info_id: row.try_get("device_info_id")?,
            quantity: row.try_get("quantity")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ShipmentRecord {
    #[serde(rename = "ID")]
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
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for ShipmentRecord {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(ShipmentRecord {
            id: row.try_get("id")?,
            shipment_date: row.try_get("shipment_date")?,
            technician: row.try_get("technician")?,
            customer_name: row.try_get("customer_name")?,
            customer_phone: row.try_get("customer_phone")?,
            customer_address: row.try_get("customer_address")?,
            tracking_number: row.try_get("tracking_number")?,
            status: row.try_get("status")?,
            description: row.try_get("description")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct SignalDelayWaringParam {
    #[serde(rename = "ID")]
    pub id: u64,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub device_uid: Option<u64>,
    pub name: Option<String>,
    pub signal_name: Option<String>,
    pub signal_id: Option<u64>,
    pub signal_delay_waring_id: Option<u64>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for SignalDelayWaringParam {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(SignalDelayWaringParam {
            id: row.try_get("id")?,
            protocol: row.try_get("protocol")?,
            identification_code: row.try_get("identification_code")?,
            device_uid: row.try_get("device_uid")?,
            name: row.try_get("name")?,
            signal_name: row.try_get("signal_name")?,
            signal_id: row.try_get("signal_id")?,
            signal_delay_waring_id: row.try_get("signal_delay_waring_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct SignalDelayWaring {
    #[serde(rename = "ID")]
    pub id: u64,
    pub name: Option<String>,
    pub script: Option<String>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for SignalDelayWaring {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(SignalDelayWaring {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            script: row.try_get("script")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignalWaringConfig {
    #[serde(rename = "ID")]
    pub id: u64,
    pub signal_id: Option<u64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub in_or_out: Option<u64>,
    pub protocol: Option<String>,
    pub identification_code: Option<String>,
    pub device_uid: Option<u64>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for SignalWaringConfig {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(SignalWaringConfig {
            id: row.try_get("id")?,
            signal_id: row.try_get("signal_id")?,
            min: row.try_get("min")?,
            max: row.try_get("max")?,
            in_or_out: row.try_get("in_or_out")?,
            protocol: row.try_get("protocol")?,
            identification_code: row.try_get("identification_code")?,
            device_uid: row.try_get("device_uid")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Signal {
    #[serde(rename = "ID")]
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
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for Signal {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Signal {
            id: row.try_get("id")?,
            protocol: row.try_get("protocol")?,
            identification_code: row.try_get("identification_code")?,
            device_uid: row.try_get("device_uid")?,
            name: row.try_get("name")?,
            alias: row.try_get("alias")?,
            signal_type: row.try_get("signal_type")?,
            unit: row.try_get("unit")?,
            cache_size: row.try_get("cache_size")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimCard {
    #[serde(rename = "ID")]
    pub id: u64,
    pub access_number: String,
    pub iccid: String,
    pub imsi: String,
    pub operator: String,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub expiration: Option<chrono::NaiveDateTime>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for SimCard {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(SimCard {
            id: row.try_get("id")?,
            access_number: row.try_get("access_number")?,
            iccid: row.try_get("iccid")?,
            imsi: row.try_get("imsi")?,
            operator: row.try_get("operator")?,
            expiration: row.try_get("expiration")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimUseHistory {
    #[serde(rename = "ID")]
    pub id: u64,
    pub sim_id: Option<u64>,
    pub device_info_id: Option<u64>,
    pub description: Option<String>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for SimUseHistory {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(SimUseHistory {
            id: row.try_get("id")?,
            sim_id: row.try_get("sim_id")?,
            device_info_id: row.try_get("device_info_id")?,
            description: row.try_get("description")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TcpHandler {
    #[serde(rename = "ID")]
    pub id: u64,
    pub device_info_id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub script: Option<String>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for TcpHandler {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(TcpHandler {
            id: row.try_get("id")?,
            device_info_id: row.try_get("device_info_id")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            name: row.try_get("name")?,
            script: row.try_get("script")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct UserBindDeviceInfo {
    #[serde(rename = "ID")]
    pub id: u64,
    pub user_id: Option<u64>,
    pub device_id: Option<u64>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for UserBindDeviceInfo {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(UserBindDeviceInfo {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            device_id: row.try_get("device_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct UserDept {
    #[serde(rename = "ID")]
    pub id: u64,
    pub user_id: Option<u64>,
    pub dept_id: Option<u64>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for UserDept {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(UserDept {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            dept_id: row.try_get("dept_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRole {
    #[serde(rename = "ID")]
    pub id: u64,
    pub user_id: Option<u64>,
    pub role_id: Option<u64>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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

impl FromRow<'_, sqlx::mysql::MySqlRow> for UserRole {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(UserRole {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            role_id: row.try_get("role_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct WebSocketHandler {
    #[serde(rename = "ID")]
    pub id: u64,
    pub device_info_id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub script: Option<String>,
    #[serde(
        rename = "CreatedAt",
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
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
impl FromRow<'_, sqlx::mysql::MySqlRow> for WebSocketHandler {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(WebSocketHandler {
            id: row.try_get("id")?,
            device_info_id: row.try_get("device_info_id")?,
            name: row.try_get("name")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            script: row.try_get("script")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

pub fn serialize_naive_datetime<S>(
    naive: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match naive {
        Some(ndt) => {
            let dt = DateTime::<Utc>::from_utc(*ndt, Utc);
            dt.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_naive_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let dt: Option<DateTime<Utc>> = Option::deserialize(deserializer)?;
    Ok(dt.map(|d| d.naive_utc()))
}
