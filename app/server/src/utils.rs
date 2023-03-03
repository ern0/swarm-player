use std::time::{SystemTime, UNIX_EPOCH};

pub const UNDEF: i64 = i64::MAX;

pub fn json_add_key(result: &mut String, value: &str) {
    json_add_quoted(result, value);
    result.push_str(":");
}

pub fn json_add_quoted(result: &mut String, value: &str) {
    result.push_str("\"");
    result.push_str(value);
    result.push_str("\"");
}

pub fn now() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
}
