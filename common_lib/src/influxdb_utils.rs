use chrono::DateTime;
use chrono::Utc;
use influxdb2::models::{DataPoint, Query};
use influxdb2::Client;
use std::collections::HashMap;
use std::error::Error;

use crate::models::DataValue;
use futures::prelude::*;
use influxdb2::api::query::FluxRecord;

pub struct InfluxDBManager {
    client: Client,
    bucket: String,
}

impl InfluxDBManager {
    pub fn new(host: &str, port: u16, org: &str, token: &str, bucket: &str) -> Self {
        let relHost = format!("http://{}:{}", host, port);

        InfluxDBManager {
            client: Client::new(relHost, org, token),
            bucket: bucket.to_string(),
        }
    }

    pub async fn write(
        &self,
        kv: HashMap<String, DataValue>,
        measurement: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut point = DataPoint::builder(measurement);
        for (key, value) in kv {
            match value {
                DataValue::Integer(v) => {
                    point = point.field(key, v);
                }
                DataValue::Float(v) => {
                    point = point.field(key, v);
                }
                DataValue::Text(v) => {
                    point = point.field(key, v);
                }
            }
        }

        let data_point = point.build()?;
        self.client
            .write(&self.bucket, stream::iter(vec![data_point]))
            .await?;
        Ok(())
    }

    pub async fn query_raw(
        &self,
        measurement: &str,
        start: DateTime<Utc>,
        stop: DateTime<Utc>,
    ) -> Result<Vec<FluxRecord>, Box<dyn Error>> {
        let flux_query = format!(
            "from(bucket: \"{}\")
            |> range(start: {}, stop: {})
            |> filter(fn: (r) => r._measurement == \"{}\")",
            self.bucket,
            start.timestamp(),
            stop.timestamp(),
            measurement
        );

        let query = Query::new(flux_query);
        // let response:Vec<Series>  = self.client.query(Some(query)).await?;

        let response = self.client.query_raw(Some(query)).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use std::collections::HashMap;
    use std::env;
    #[tokio::test]
    async fn test_influxdb_operations() -> Result<(), Box<dyn std::error::Error>> {
        // 设置环境变量（在实际测试中，你可能需要确保这些变量被正确设置）
        env::set_var("INFLUXDB_HOST", "http://localhost:8086");
        env::set_var("INFLUXDB_ORG", "myorg");
        env::set_var("INFLUXDB_TOKEN", "lVXFhDO4rOGqfc5Hpr9MHtbiEQyJMoEmlH8LbIwta41QYB-9A_H9d6cCpfUnaLGuQiC_RbH93QGFlpPeukGX-Q==");

        let host = env::var("INFLUXDB_HOST").unwrap();
        let org = env::var("INFLUXDB_ORG").unwrap();
        let token = env::var("INFLUXDB_TOKEN").unwrap();
        let bucket = "aaa"; // 可以替换成实际的 bucket 名称

        let db_manager = InfluxDBManager::new("localhost", 8086, &org, &token, bucket);

        let measurement = "sb";
        // 准备键值对并写入 CPU 使用数据
        let mut tags = HashMap::new();
        tags.insert("age".to_string(), DataValue::Integer(10));
        tags.insert("weight".to_string(), DataValue::Float(12.1));

        db_manager.write(tags, measurement).await?;
        println!("written successfully.");

        let end_time = Utc::now();
        let start_time = end_time - Duration::hours(1);

        let raw_data = db_manager
            .query_raw(measurement, start_time, end_time)
            .await?;
        println!("Raw data: {:?}", raw_data);

        Ok(())
    }
}
