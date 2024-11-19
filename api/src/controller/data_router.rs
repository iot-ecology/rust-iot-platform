use crate::biz::mqtt_client_biz::MqttClientBiz;
use common_lib::config::Config;
use common_lib::influxdb_utils::InfluxDBManager;
use common_lib::models::InfluxQueryConfig;
use common_lib::ut::calc_bucket_name;
use influxdb2_structmap::GenericMap;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;
use std::collections::HashMap;

#[post("/query/influxdb", format = "json", data = "<data>")]
pub async fn query_influxdb(
    data: Json<InfluxQueryConfig>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 获取配置信息，处理可能的错误
    let influx_config = match &config.influx_config {
        Some(cfg) => cfg,
        None => return rocket::response::status::Custom(
            Status::InternalServerError,
            Json(json!({ "error": "Influx configuration is missing" })),
        )
    };

    let bucket = match &influx_config.bucket {
        Some(b) => b.clone(),
        None => return rocket::response::status::Custom(
            Status::InternalServerError,
            Json(json!({ "error": "Bucket configuration is missing" })),
        )
    };

    let protocol = match &data.protocol {
        Some(p) => p.clone(),
        None => return rocket::response::status::Custom(
            Status::BadRequest,
            Json(json!({ "error": "Protocol is required" })),
        )
    };

    let device_uid = match data.device_uid {
        Some(uid) => uid,
        None => return rocket::response::status::Custom(
            Status::BadRequest,
            Json(json!({ "error": "Device UID is required" })),
        )
    };

    // 计算 bucket 名称并生成查询
    let mut data_inner = data.into_inner();
    data_inner.bucket = calc_bucket_name(bucket.as_str(), protocol.as_str(), device_uid);
    let query_string = data_inner.generate_flux_query();


    let manager = InfluxDBManager::new(
        &influx_config.clone().host.unwrap(),
        influx_config.clone().port.unwrap(),
        &influx_config.clone().org.unwrap(),
        &influx_config.clone().token.unwrap(),
    );

    // 执行查询并处理结果
    let query_result = match manager.query_with_string(query_string).await {
        Ok(result) => result,
        Err(e) => return rocket::response::status::Custom(
            Status::InternalServerError,
            Json(json!({ "error": format!("Query failed: {}", e) })),
        )
    };

    let mut records: Vec<GenericMap> = Vec::new();
    for row in query_result {
        let values = row.values;
        records.push(values);
    }

    // 按字段分组
    let grouped_data = group_by_field(&records);

    rocket::response::status::Custom(
        Status::Ok,
        Json(json!({ "data": grouped_data })),
    )
}

fn group_by_field(records: &[GenericMap]) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
    let mut grouped: HashMap<String, Vec<HashMap<String, serde_json::Value>>> = HashMap::new();

    for record in records {
        if let Some(field) = record.get("_field").and_then(|v| {
            if let influxdb2_structmap::value::Value::String(s) = v {
                Some(s.as_str())
            } else {
                None
            }
        }) {
            let record_map: HashMap<String, serde_json::Value> = record.iter()
                .map(|(k, v)| {
                    let json_value = match v {
                        influxdb2_structmap::value::Value::String(s) => serde_json::Value::String(s.clone()),
                        influxdb2_structmap::value::Value::Double(i) => serde_json::Value::String(i.to_string()),
                        influxdb2_structmap::value::Value::Long(f) => serde_json::Value::Number(serde_json::Number::from(*f)),
                        influxdb2_structmap::value::Value::Bool(b) => serde_json::Value::Bool(*b),
                        _ => serde_json::Value::Null,
                    };
                    (k.clone(), json_value)
                })
                .collect();

            grouped.entry(field.to_string())
                .or_insert_with(Vec::new)
                .push(record_map);
        }
    }

    grouped
}

#[post("/query/QueryMeasurement")]
pub async fn query_measurement(
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/query/str-influxdb", format = "json", data = "<data>")]
pub async fn query_influxdb_string(
    data: Json<InfluxQueryConfig>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 获取配置信息，处理可能的错误
    let influx_config = match &config.influx_config {
        Some(cfg) => cfg,
        None => return rocket::response::status::Custom(
            Status::InternalServerError,
            Json(json!({ 
                "status": "error",
                "message": "Influx configuration is missing" 
            })),
        )
    };

    let bucket = match &influx_config.bucket {
        Some(b) => b.clone(),
        None => return rocket::response::status::Custom(
            Status::InternalServerError,
            Json(json!({ 
                "status": "error",
                "message": "Bucket configuration is missing" 
            })),
        )
    };

    let protocol = match &data.protocol {
        Some(p) => p.clone(),
        None => return rocket::response::status::Custom(
            Status::BadRequest,
            Json(json!({ 
                "status": "error",
                "message": "Protocol is required" 
            })),
        )
    };

    let device_uid = match data.device_uid {
        Some(uid) => uid,
        None => return rocket::response::status::Custom(
            Status::BadRequest,
            Json(json!({ 
                "status": "error",
                "message": "Device UID is required" 
            })),
        )
    };

    // 计算 bucket 名称并生成查询
    let mut data_inner = data.into_inner();
    data_inner.bucket = calc_bucket_name(bucket.as_str(), protocol.as_str(), device_uid);
    let query_string = data_inner.generate_flux_query_string();

    let manager = InfluxDBManager::new(
        &influx_config.clone().host.unwrap(),
        influx_config.clone().port.unwrap(),
        &influx_config.clone().org.unwrap(),
        &influx_config.clone().token.unwrap(),
    );

    // 执行查询并处理结果
    let query_result = match manager.query_with_string(query_string).await {
        Ok(result) => result,
        Err(e) => return rocket::response::status::Custom(
            Status::InternalServerError,
            Json(json!({ 
                "status": "error",
                "message": format!("Query failed: {}", e) 
            })),
        )
    };

    // 将查询结果转换为 Vec<HashMap>
    let mut records: Vec<GenericMap> = Vec::new();
    for row in query_result {
        // let values = row.values();
        // records.push(values);
        records.push(row.values)
    }

    // 按字段分组
    let grouped_data = group_by_field(&records);

    // 返回成功结果
    rocket::response::status::Custom(
        Status::Ok,
        Json(json!({
            "status": "success",
            "data": grouped_data
        })),
    )
}
