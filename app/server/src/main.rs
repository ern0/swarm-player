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
use crate::channel_manager::ChannelManager;
use crate::reporting::Reporting;

pub fn main() {

    let client_list = create_client_list();
    let reporting = Arc::new(Reporting::new(client_list.clone()));

    ChannelManager::new(client_list.clone());
    ClientManager::new(client_list.clone(), reporting.clone()).start();

    reporting.start();

    loop { sleep(Duration::from_secs(1)); }
}

fn create_client_list() -> SharedClientList {

    let clients_hash_map: HashMap<u64, SharedClient> = HashMap::new();
    let clients_lock = RwLock::new(clients_hash_map);
    let clients = Arc::new(clients_lock);

    return clients;
}
