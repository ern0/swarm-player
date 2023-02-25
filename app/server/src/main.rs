#![allow(unused)]

mod client_manager;
mod client;

use simple_websockets;
use client_manager::ClientManager;
use std::time::{ Duration };
use std::thread::{ sleep, spawn };
use std::sync::{ Mutex, Arc };
use std::rc::Rc;

pub fn main() {

    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");

    let client_manager = ClientManager::new(event_hub);
    let cm1 = Arc::new(Mutex::new(client_manager));
    let cm2 = cm1.clone();

    spawn(move || cm1.lock().unwrap().run() );

    loop {
        cm2.lock().unwrap().broadcast(String::from("hello"));
        sleep(Duration::from_millis(1000));
    }

}
