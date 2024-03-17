use std::time::{SystemTime, Duration, UNIX_EPOCH};

pub type Port = u16;
pub type ClientSessionId = u64;

pub const UNDEF: i64 = i64::MAX;
pub const STAMP_OFFSET_MS: i64 = 500;  //TODO: change to prod value

pub fn systime_to_millis(stamp: SystemTime) -> i64 {
  	stamp
        .duration_since(UNIX_EPOCH).unwrap()
        .as_millis() as i64
}

pub fn now_millis() -> i64 {
    let stamp = SystemTime::now();
    systime_to_millis(stamp)
}
