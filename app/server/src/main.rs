mod utils;
mod client_manager;
mod channel_manager;
mod reporting;
mod client;
mod packet;

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::Duration;
use std::thread::sleep;
use utils::SharedClientList;

use crate::utils::SharedClient;
use crate::client_manager::ClientManager;

pub fn main() {

    let client_list = create_client_list();
    let client_manager = ClientManager::new(client_list);
    client_manager.start();

    loop { sleep(Duration::from_secs(1)); }
}

fn create_client_list() -> SharedClientList {

    let client_list_hash_map: HashMap<u64, SharedClient> = HashMap::new();
    let client_list_lock = RwLock::new(client_list_hash_map);
    let client_list = Arc::new(client_list_lock);

    return client_list;
}
