pub fn calc_bucket_name(prefix: &str, protocol: &str, id: u32) -> String {
    format!("{}_{}_{}", prefix, protocol, id % 100)
}


pub fn calc_collection_name(prefix: &str, id: i32) -> String {
    let string = format!("{}_{}", prefix, id % 100);
    return string;
}
