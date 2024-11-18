pub fn calc_bucket_name(prefix: &str, protocol: &str, id: u64) -> String {
    format!("{}_{}_{}", prefix, protocol, id % 100)
}


pub fn calc_collection_name(prefix: &str, id: i32) -> String {
    let string = format!("{}_{}", prefix, id % 100);
    return string;
}
pub fn calc_measurement(device_uid: &str, identification_code: &str, protocol: &str) -> String {
    format!("{}_{}_{}", protocol, device_uid, identification_code)
}
