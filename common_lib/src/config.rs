/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use tokio::sync::{Mutex, MutexGuard, OnceCell};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeInfo {
    pub host: String,
    pub port: u16,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub size: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]

pub struct MqConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]

pub struct InfluxConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub token: Option<String>,
    pub org: Option<String>,
    pub bucket: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoConfig {
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: Option<String>,
    pub collection: Option<String>,
    pub waring_collection: Option<String>,
    pub script_waring_collection: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub node_info: NodeInfo,
    pub redis_config: RedisConfig,
    pub mq_config: MqConfig,
    pub influx_config: Option<InfluxConfig>,
    pub mongo_config: Option<MongoConfig>,
    pub mysql_config: Option<MySQLConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MySQLConfig {
    pub username: String,
    pub password: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    pub port: u16,
    pub dbname: String,
}

// 全局配置实例
static CONFIG_INSTANCE: OnceCell<Mutex<Config>> = OnceCell::const_new();

pub async fn read_config(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;

    // 将配置存储到全局变量中
    CONFIG_INSTANCE
        .set(Mutex::new(config))
        .map_err(|_| "Config instance already initialized")?;

    Ok(())
}

pub fn read_config_tb(file_path: &str) -> Config {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = serde_yaml::from_str(&contents).unwrap();

    config
}

pub async fn get_config() -> Result<MutexGuard<'static, Config>, Box<dyn std::error::Error>> {
    let instance = CONFIG_INSTANCE
        .get()
        .ok_or("Config instance not initialized")?;
    let guard = instance.lock().await;
    Ok(guard)
}
