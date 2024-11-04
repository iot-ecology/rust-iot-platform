use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use tokio::sync::{Mutex, MutexGuard, OnceCell};

#[derive(Debug, Deserialize, Clone)]
pub struct NodeInfo {
    pub host: String,
    pub port: u16,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub size: u8,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub node_info: NodeInfo,
    pub redis_config: RedisConfig,
    pub mq_config: MqConfig,
    pub influx_config: Option<InfluxConfig>,
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

pub async fn get_config() -> Result<MutexGuard<'static, Config>, Box<dyn std::error::Error>> {
    let instance = CONFIG_INSTANCE
        .get()
        .ok_or("Config instance not initialized")?;
    let guard = instance.lock().await;
    Ok(guard)
}
