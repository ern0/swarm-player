mod utils;
mod client_manager;
mod client;
mod packet;

use std::time::Duration;
use std::thread::sleep;
use crate::client_manager::ClientManager;

pub fn main() {
    ClientManager::new().start();
    loop { sleep(Duration::from_secs(1)); }
}
