#![allow(unused)]

use std::process::exit;
use std::thread::spawn;
use simple_websockets::{Event, Message, Responder};

use crate::logger::Logger;
use crate::utils::{Port, ClientSessionId};
use crate::client_manager::ClientManager;

pub struct WebServer {
    port: Port,
    logger: Logger,
    client_manager: ClientManager,
}

impl WebServer {
    pub fn new(port: Port, logger:Logger, client_manager: ClientManager) -> Self {
        Self {
            port,
            logger,
            client_manager,
        }
    }

    pub fn start(mut self) {

        let event_hub_result = simple_websockets::launch(self.port);
        if let Result::Err(error) = &event_hub_result {
            self.logger.log_webserver_fail(self.port, error);
            exit(1);
        }
        let event_hub = event_hub_result.unwrap();
        self.logger.log_webserver_okay(self.port);

        spawn(move || self.run(event_hub));
    }

    pub fn run(&mut self, event_hub: simple_websockets::EventHub) {

        loop {
            match event_hub.poll_event() {
                Event::Connect(client_session_id, responder) => {
                    self.client_manager.on_client_connect(client_session_id, responder);
                }
                Event::Disconnect(client_session_id) => {
                    self.client_manager.on_client_disconnect(client_session_id);
                }
                Event::Message(client_session_id, message) => {
                    //self.on_message(client_id, message);
                }
            }
        }
    }

}
