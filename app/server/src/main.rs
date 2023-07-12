mod utils;
mod client_manager;
mod client;
mod packet;

use client_manager::ClientManager;
use simple_websockets;
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;

pub fn main() {
    
    let event_hub = simple_websockets::launch(8080).expect("failed to listen on port 8080");
    println!("server is up");

    let client_manager = ClientManager::new(event_hub);
    let cm1 = Arc::new(client_manager);
    let cm2 = cm1.clone();

    spawn(move || cm1.run());

    loop {
        cm2.broadcast();
        sleep(Duration::from_secs(1));
    }
}
