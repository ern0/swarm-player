#[allow(unused)]

mod client_manager;
mod client;

use simple_websockets;
use client_manager::ClientManager;
use std::thread;

pub fn main() {

    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");

    let client_manager = ClientManager::new(event_hub);
    //thread::spawn(|| client_manager.run() );

    client_manager.broadcast(String::from("hello"));

}
