mod client_manager;
mod client;

use simple_websockets;
use client_manager::ClientManager;

pub fn main() {

    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");

    let mut client_manager = ClientManager::new(event_hub);
    client_manager.run();

}
