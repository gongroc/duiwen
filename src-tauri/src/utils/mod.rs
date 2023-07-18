use chrono::Utc;

pub fn get_string_current_timestamp() -> String {
    format!("{}", Utc::now().format("%Y-%m-%d %H:%M:%S"))
}