#![allow(unused)]

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::prelude::{Utc, Local, DateTime};
use std::sync::{Arc, RwLock};
use crate::client::Client;

pub const STAMP_OFFSET_MS: i64 = 500;  //TODO: change to prod value
pub const UNDEF: i64 = i64::MAX;
pub type ClientList = Arc<RwLock<HashMap<u64, Client>>>;

pub fn systime_to_millis(stamp: SystemTime) -> i64 {

    return stamp
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
}

pub fn now_millis() -> i64 {
    let stamp = SystemTime::now();
    return systime_to_millis(stamp);
}

pub fn systime_to_string(stamp: SystemTime) -> String {

    let datetime: DateTime<Local> = stamp.into();
    return datetime.format("%T").to_string();
}

pub fn now_string() -> String {
    let stamp = SystemTime::now();
    return systime_to_string(stamp);
}
