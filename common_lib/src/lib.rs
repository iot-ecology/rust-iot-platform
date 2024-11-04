mod influxdb_utils;
pub mod models;
pub mod protocol_config;
pub mod rabbit_utils;
pub mod redis_handler;

pub fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}
