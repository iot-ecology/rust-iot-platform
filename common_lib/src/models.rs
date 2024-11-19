use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
pub struct DataRowList {
    pub Time: i64,                  // 秒级时间戳
    pub DeviceUid: String,          // 能够产生网络通讯的唯一编码
    pub IdentificationCode: String, // 设备标识码
    pub DataRows: Vec<DataRow>,     // 数据行
    pub Nc: String,                 // Nc 字段
    #[serde(skip_serializing_if = "Option::is_none")] // 如果为 None，则不序列化
    pub Protocol: Option<String>, // 协议字段
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MQTTMessage {
    #[serde(rename = "mqtt_client_id")]
    pub mqtt_client_id: String,
    pub message: String,
}

impl MQTTMessage {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize MQTTMessage")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataRow {
    pub Name: String,  // 数据行名称
    pub Value: String, // 数据行值
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpMessage {
    pub uid: String,
    pub message: String,
}

impl HttpMessage {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize HttpMessage")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsMessage {
    pub uid: String,
    pub message: String,
}

impl WsMessage {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize WsMessage")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoapMessage {
    pub uid: String,
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MqttConfig {
    #[serde(rename = "broker")]
    pub broker: String,

    #[serde(rename = "port")]
    pub port: i32,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "sub_topic")]
    pub sub_topic: String,

    #[serde(rename = "client_id")]
    pub client_id: String,
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
    pub cache_size: i64,
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
    pub id: i64, // ID
}

#[derive(Debug)]
pub struct SignalMapping {
    pub cache_size: i64,
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
    pub id: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Tv {
    pub time: i64,
    pub value: f64,
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InfluxQueryConfig {
    pub bucket: String,
    pub measurement: String,
    pub fields: Vec<String>,
    pub start_time: i64,
    pub end_time: i64,
    pub aggregation: AggregationConfig,
    pub reduce: String, // sum, min, max, mean
    #[serde(rename = "device_uid")]
    pub device_uid: Option<i64>,

    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AggregationConfig {
    pub every: i32,
    pub function: String, // e.g., "mean", "sum", "min", "max"
    pub create_empty: bool,
}

impl InfluxQueryConfig {

    pub fn generate_flux_query_string(&self) -> String {
        // 构建字段过滤条件
        let filters: Vec<String> = self.fields
            .iter()
            .map(|field| format!(r#"r["_field"] == "{}""#, field))
            .collect();

        // 构建时间范围条件
        let time_range = if self.start_time != 0 && self.end_time != 0 {
            format!("start: {}, stop: {}", self.start_time, self.end_time)
        } else {
            String::new()
        };

        // 构建过滤子句
        let filter_clause = if !filters.is_empty() {
            format!("|> filter(fn: (r) => {})", filters.join(" or "))
        } else {
            String::new()
        };

        // 构建完整的 Flux 查询，与 Go 版本完全一致
        format!(
            r#"
        from(bucket: "{}")
            |> range({})
            {}
            |> filter(fn: (r) => r["_measurement"] == "{}")
            |> aggregateWindow(every: {}s, fn: {}, createEmpty: {})
            |> yield(name: "first")
    "#,
            self.bucket,
            time_range,
            filter_clause,
            self.measurement,
            self.aggregation.every,
            self.aggregation.function,
            self.aggregation.create_empty
        ).trim().to_string()
    }

    // Generate a Flux query based on the InfluxQueryConfig
    pub fn generate_flux_query(&self) -> String {
        let filters: Vec<String> = self
            .fields
            .iter()
            .map(|field| format!(r#"r["_field"] == "{}""#, field))
            .collect();

        let time_range = if self.start_time != 0 && self.end_time != 0 {
            format!(r#"start: {}, stop: {}"#, self.start_time, self.end_time)
        } else {
            String::new()
        };

        let filter_clause = if !filters.is_empty() {
            format!(r#"|> filter(fn: (r) => {})"#, filters.join(" or "))
        } else {
            String::new()
        };

        format!(
            r#"
            from(bucket: "{}")
                |> range({})
                {}
                |> filter(fn: (r) => r["_measurement"] == "{}")
                |> aggregateWindow(every: {}s, fn: {}, createEmpty: {})
                |> yield(name: "mean")
            "#,
            self.bucket,
            time_range,
            filter_clause,
            self.measurement,
            self.aggregation.every,
            self.aggregation.function,
            self.aggregation.create_empty
        )
    }

    // Generate a Flux reduce query based on the InfluxQueryConfig
    pub fn generate_flux_reduce(&self) -> String {
        let filters: Vec<String> = self
            .fields
            .iter()
            .map(|field| format!(r#"r["_field"] == "{}""#, field))
            .collect();

        let time_range = if self.start_time != 0 && self.end_time != 0 {
            format!(r#"start: {}, stop: {}"#, self.start_time, self.end_time)
        } else {
            String::new()
        };

        let filter_clause = if !filters.is_empty() {
            format!(r#"|> filter(fn: (r) => {})"#, filters.join(" or "))
        } else {
            String::new()
        };

        format!(
            r#"
            from(bucket: "{}")
                |> range({})
                {}
                |> filter(fn: (r) => r["_measurement"] == "{}")
                |> {}()
            "#,
            self.bucket, time_range, filter_clause, self.measurement, self.reduce
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_flux_query() {
        let aggregation_config = AggregationConfig {
            every: 60,
            function: String::from("mean"),
            create_empty: true,
        };

        let query_config = InfluxQueryConfig {
            bucket: String::from("my_bucket"),
            measurement: String::from("temperature"),
            fields: vec![String::from("value")],
            start_time: 1627848123,
            end_time: 1627851723,
            aggregation: aggregation_config,
            reduce: String::from("sum"),
            device_uid: None,
            protocol: None,
        };

        let flux_query = query_config.generate_flux_query();
        println!("Generated Flux Query:\n{}", flux_query);

        assert!(flux_query.contains("from(bucket: \"my_bucket\")"));
        assert!(flux_query.contains("|> range(start: 1627848123, stop: 1627851723)"));
        assert!(flux_query.contains("|> aggregateWindow(every: 60s, fn: mean, createEmpty: true)"));
    }

    #[test]
    fn test_generate_flux_reduce() {
        let aggregation_config = AggregationConfig {
            every: 60,
            function: String::from("mean"),
            create_empty: true,
        };

        let query_config = InfluxQueryConfig {
            bucket: String::from("my_bucket"),
            measurement: String::from("temperature"),
            fields: vec![String::from("value")],
            start_time: 1627848123,
            end_time: 1627851723,
            aggregation: aggregation_config,
            reduce: String::from("sum"),
            device_uid: None,
            protocol: None,
        };

        let flux_reduce_query = query_config.generate_flux_reduce();
        println!("Generated Flux Reduce Query:\n{}", flux_reduce_query);

        assert!(flux_reduce_query.contains("from(bucket: \"my_bucket\")"));
        assert!(flux_reduce_query.contains("|> range(start: 1627848123, stop: 1627851723)"));
        assert!(flux_reduce_query.contains("|> sum()"));
    }
}
