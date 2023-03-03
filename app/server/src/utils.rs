use std::time::{SystemTime, UNIX_EPOCH};

pub const UNDEF: i64 = i64::MAX;

pub fn now() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
}
