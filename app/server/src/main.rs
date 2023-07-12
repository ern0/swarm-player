#![allow(unused)]

mod utils;
mod client_manager;
mod client;
mod packet;

use simple_websockets;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;
use crate::client_manager::ClientManager;
use crate::packet::Packet;

pub fn main() {
    
    let event_hub = simple_websockets::launch(8080).expect("failed to listen on port 8080");
    println!("server is up");

    let (sender, receiver,) = mpsc::channel();

    let client_manager = ClientManager::new());
    let cm0 = Arc::new(client_manager);
    let cm1 = cm0.clone();
    let cm2 = cm1.clone();

    spawn(move || cm1.run_event_hub(&event_hub));
    spawn(move || cm2.run_broadcast(&receiver));

    let mut counter = 0;
    loop {        
        let mut packet = Packet::new_simple_num("DISPLAY", counter);    
        packet.set_num(0, counter);
        sender.send(packet).unwrap();
        counter += 1;

        sleep(Duration::from_secs(1));
    }

}
