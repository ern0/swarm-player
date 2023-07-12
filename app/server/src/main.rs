#![allow(unused)]

mod client_manager;
mod client;

use simple_websockets;
use client_manager::ClientManager;
use std::thread;
use std::sync::{ Mutex, Arc };
use std::rc::Rc;

pub fn main() {

    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");

    let client_manager = ClientManager::new(event_hub);
    let cm1 = Arc::new(client_manager);
    let cm2 = cm1.clone();

    //thread::spawn(move || cm1.run() );

    thread::spawn(move || cm1.broadcast(String::from("hello")) );
    cm2.broadcast(String::from("hello"));

}
