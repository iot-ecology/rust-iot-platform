use serde::Deserialize;
use std::fs::File;
use std::io::Read;


#[derive(Debug, Deserialize)]
pub struct NodeInfo {
    pub host: String,
    pub port: u16,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub size: u8,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct MqConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub node_info: NodeInfo,
    pub redis_config: RedisConfig,
    pub mq_config: MqConfig,
}

pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

