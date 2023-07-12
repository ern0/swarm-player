mod client_manager;
mod client;

use simple_websockets;
use client_manager::ClientManager;
use std::time::{ Duration };
use std::thread::{ sleep, spawn };
use std::sync::Arc;

pub fn main() {

    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");
    println!("Server is up");

    let client_manager = ClientManager::new(event_hub);
    let cm1 = Arc::new(client_manager);
    let cm2 = cm1.clone();

    spawn(move || cm1.run());

    loop {
        cm2.broadcast();
        sleep(Duration::from_secs(1));
    }

}
