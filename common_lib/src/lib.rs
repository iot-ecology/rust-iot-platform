pub mod config;
pub mod influxdb_utils;
pub mod models;
pub mod mongo_utils;
pub mod rabbit_utils;
pub mod redis_handler;
pub mod time_utils;

pub fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}
