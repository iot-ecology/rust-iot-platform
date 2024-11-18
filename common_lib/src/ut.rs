pub fn calc_bucket_name(prefix: &str, protocol: &str, id: u32) -> String {
    format!("{}_{}_{}", prefix, protocol, id % 100)
}
