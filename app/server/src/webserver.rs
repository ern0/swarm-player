#![allow(unused)]

use std::process::exit;
use std::thread::spawn;
use simple_websockets::{Event, Message, Responder};

use crate::utils::{ClientSessionId};

pub struct WebServer {
    port: u16,
}

impl WebServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
        }
    }

    pub fn start(mut self) {

        let event_hub_result = simple_websockets::launch(self.port);
        if let Result::Err(error) = &event_hub_result {
            println!(
                "[mgr]: failed to launch server on port {}: {:?}",
                self.port,
                error,
            );
            exit(1);
        }
        let event_hub = event_hub_result.unwrap();

        println!(
            "[mgr]: server is up, listening on port {}",
            self.port,
        );

        spawn(move || self.run(event_hub));
    }

    pub fn run(&mut self, event_hub: simple_websockets::EventHub) {

        loop {
            match event_hub.poll_event() {
                Event::Connect(client_session_id, responder) => {
                    self.on_connect(client_session_id, responder);
                }
                Event::Disconnect(client_session_id) => {
                    //self.on_disconnect(client_id);
                }
                Event::Message(client_session_id, message) => {
                    //self.on_message(client_id, message);
                }
            }
        }
    }

    fn on_connect(&mut self, client_session_id: ClientSessionId, responder: Responder) {
    }
}
