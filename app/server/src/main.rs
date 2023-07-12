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

    let client_manager = ClientManager::new(event_hub);
    client_manager.start();

    // let mut counter = 0;
    // loop {        
    //     let mut packet = Packet::new_simple_num("DISPLAY", counter);
    //     packet.set_num(0, counter);
    //     client_manager.broadcast(&packet);
    //     counter += 1;

    //     sleep(Duration::from_secs(1));
    // }

}
