use chrono::DateTime;
use chrono::Utc;
use influxdb2::models::{Bucket, DataPoint, PostBucketRequest, Query};
use influxdb2::Client;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use crate::models::DataValue;
use futures::prelude::*;
use influxdb2::api::query::FluxRecord;
use log::info;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;

pub struct InfluxDBManager {
   pub client: Client,
    host: String,
    port: u16,
    org: String,
    token: String,
}

impl InfluxDBManager {
    pub fn new(host: &str, port: u16, org: &str, token: &str) -> Self {
        let rel_host = format!("http://{}:{}", host, port);

        InfluxDBManager {
            client: Client::new(rel_host, org, token),
            host: host.to_string(),
            port: port,
            org: org.to_string(),
            token: token.to_string(),
        }
    }
    pub async fn bucket_exists(&self, bucket_name: &str) -> Result<bool, Box<dyn Error>> {
        let buckets = self.client.list_buckets(None).await?;
        Ok(buckets.buckets.iter().any(|b| b.name == bucket_name))
    }

    // 创建 bucket
    pub async fn create_bucket(&self, name: String) -> Result<(), Box<dyn Error>> {
        if self.bucket_exists(name.as_str()).await? {
            info!("Bucket '{}' already exists.", name);
            return Ok(());
        }

        self.client
            .create_bucket(Some(PostBucketRequest {
                org_id: self.org.clone(),
                name: name.clone(),
                description: None,
                rp: None,
                retention_rules: vec![],
            }))
            .await?;
        info!("Bucket '{}' created successfully.", name);
        Ok(())
    }
    pub async fn write(
        &self,
        kv: HashMap<String, DataValue>,
        measurement: &str,
        bucket: &str,
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
            .write(bucket, stream::iter(vec![data_point]))
            .await?;
        Ok(())
    }

    pub async fn query_raw(
        &self,
        measurement: &str,
        start: DateTime<Utc>,
        stop: DateTime<Utc>,
        bucket: &str,
    ) -> Result<Vec<FluxRecord>, Box<dyn Error>> {
        let flux_query = format!(
            "from(bucket: \"{}\")
            |> range(start: {}, stop: {})
            |> filter(fn: (r) => r._measurement == \"{}\")",
            bucket,
            start.timestamp(),
            stop.timestamp(),
            measurement
        );

        let query = Query::new(flux_query);
        let response = self.client.query_raw(Some(query)).await?;

        Ok(response)
    }

    pub async fn query_with_string(
        &self,
        flux_query: String,
    ) -> Result<Vec<FluxRecord>, Box<dyn Error>> {
        let query = Query::new(flux_query);

        let response = self.client.query_raw(Some(query)).await?;

        Ok(response)
    }
}


#[derive(Debug)]
pub enum LocValue {
      Map(HashMap<i64, f64>),
     Scalar(f64),
}

impl Serialize for LocValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LocValue::Map(map) => {
                let mut ser_map = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    ser_map.serialize_entry(k, v)?;
                }
                ser_map.end()
            }
            LocValue::Scalar(scalar) => serializer.serialize_f64(*scalar),
        }
    }
}

 impl<'de> Deserialize<'de> for LocValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LocValueVisitor;

        impl<'de> Visitor<'de> for LocValueVisitor {
            type Value = LocValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map or a scalar value")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LocValue::Scalar(value))
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some((key, value)) = access.next_entry()? {
                    map.insert(key, value);
                }
                Ok(LocValue::Map(map))
            }
        }

        deserializer.deserialize_any(LocValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, Utc};
    use influxdb2_structmap::value::Value;
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

        let db_manager = InfluxDBManager::new("localhost", 8086, &org, &token);

        let measurement = "sb";
        // 准备键值对并写入 CPU 使用数据
        let mut tags = HashMap::new();
        tags.insert("age".to_string(), DataValue::Integer(10));
        tags.insert("weight".to_string(), DataValue::Float(12.1));

        db_manager.write(tags, measurement, "bbb").await?;
        info!("written successfully.");

        let end_time = Local::now();

        println!("end_time: {:?}", end_time);
        let start_time = end_time - Duration::hours(1000);

        let raw_data = db_manager
            .query_raw(
                measurement,
                start_time.with_timezone(&Utc),
                end_time.with_timezone(&Utc),
                "bbb",
            )
            .await?;
        info!("Raw data: {:?}", raw_data);

        for record in raw_data {
            // 打印每条记录的详细信息
            println!("Record: {:?}", record);

            let time = record.values.get("_time").unwrap();
            match time {
                Value::Unknown => {}
                Value::String(_) => {}
                Value::Double(_) => {}
                Value::Bool(_) => {}
                Value::Long(_) => {}
                Value::UnsignedLong(_) => {}
                Value::Duration(_) => {}
                Value::Base64Binary(_) => {}
                Value::TimeRFC(tt) => {
                    let beijing_time = tt.with_timezone(&chrono::FixedOffset::east(8 * 3600));
                }
            }
            println!("Time: {:?}", time);
        }

        Ok(())
    }
}
