#![allow(unused)]

use std::thread::spawn;
use simple_websockets::{Event, Message, Responder};

pub struct WebServer {
    debug: bool,
}

impl WebServer {
    pub fn new() -> Self {
        Self {
            debug: false
        }
    }

    pub fn start(mut self) {

        let event_hub = simple_websockets::launch(8080)
            .expect("failed to listen on port 8080");

        println!(
            "[mgr]: server is up",
        );

        spawn(move || self.run(event_hub));
    }

    pub fn run(&mut self, event_hub: simple_websockets::EventHub) {

        loop {
            match event_hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    //self.on_connect(client_id, responder);
                }
                Event::Disconnect(client_id) => {
                    //self.on_disconnect(client_id);
                }
                Event::Message(client_id, message) => {
                    //self.on_message(client_id, message);
                }
            }
        }

    }
}
