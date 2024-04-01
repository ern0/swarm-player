use std::time::{SystemTime, Duration, UNIX_EPOCH};
use crate::packet::{PacketInt, PacketStamp};

pub type Port = u16;
pub type ClientSessionId = u64;

pub const UNDEF: PacketInt = i64::MAX;
pub const STAMP_OFFSET_MS: PacketStamp = 500;  //TODO: change to prod value

pub fn systime_to_millis(stamp: SystemTime) -> PacketStamp {
  	stamp
        .duration_since(UNIX_EPOCH).unwrap()
        .as_millis() as PacketStamp
}

pub fn now_millis() -> PacketStamp {
    let stamp = SystemTime::now();
    systime_to_millis(stamp)
}
