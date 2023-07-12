#![allow(unused)]

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, RwLock};
use crate::client::Client;

pub const STAMP_OFFSET_MS: i64 = 101;  //TODO: change to prod value
pub const UNDEF: i64 = i64::MAX;
pub type ClientList = Arc<RwLock<HashMap<u64, Client>>>;

pub fn now() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
}
