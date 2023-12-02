use chrono::Utc;
pub fn get_current_timestamp() -> String {
    let current_time = Utc::now();
    let timestamp = current_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    timestamp
}