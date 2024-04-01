#![allow(unused)]

use std::process::exit;
use std::sync::Arc;
use std::thread::spawn;
use simple_websockets::{Event, Message, Responder};

use crate::logger::Logger;
use crate::utils::{Port, ClientSessionId};
use crate::packet::Packet;
use crate::client_manager::ClientManager;

pub struct WebServer {
    pub listen_port: Port,
    pub logger: Arc<Logger>,
    pub client_manager: ClientManager,
}

impl WebServer {

    pub fn start(mut self) {

        let event_hub_result = simple_websockets::launch(self.listen_port);
        match event_hub_result {
            Ok(event_hub) => {
                self.logger.log_webserver_start_success(self.listen_port);
                spawn(move || self.run(event_hub));
            },
            Err(error) => {
                self.logger.log_webserver_start_fail(self.listen_port, &error);
                exit(1);
            },
        }
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
                    self.process_incoming_message(client_session_id, message);
                }
            }
        }
    }

    fn process_incoming_message(&self, client_session_id: ClientSessionId, message: Message) {

        match message {
            Message::Text(message_text) => {
                self.logger.log_webserver_message_received(client_session_id, &message_text);
                let packet = Packet::from(&message_text);
                self.process_incoming_packet(packet);
            },
            _ => {
                self.logger.log_webserver_message_invalid(client_session_id)
            }
        }
    }

    fn process_incoming_packet(&self, packet: Packet) {
        println!(
            "{}",
            packet.get_type(),
        );
    }
}
