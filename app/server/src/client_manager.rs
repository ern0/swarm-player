#![allow(unused)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use simple_websockets::{Event, EventHub, Message, Responder};
use crate::utils::now;
use crate::client::Client;
use crate::packet::Packet;

pub struct ClientManager {
    clients: Mutex<HashMap<u64, Client>>,
}

impl ClientManager {

    pub fn new() -> Self {

        return ClientManager {
            clients: Mutex::new(HashMap::new()),
        };
    }

    pub fn run_event_hub(&self, event_hub: &EventHub) {

        loop {

            match event_hub.poll_event() {

                Event::Connect(client_id, responder) => {
                    self.on_connect(responder, client_id);
                }

                Event::Disconnect(client_id) => {
                    self.on_disconnect(client_id);
                }

                Event::Message(client_id, message) => {
                    self.on_message(client_id, message);
                }

            }

        }

    }

    fn on_connect(&self, responder: Responder, client_id: u64) {

        println!("[{}] connected", client_id);

        let client = Client::new(client_id, responder);
        self.clients.lock().unwrap().insert(client.id, client);
    }

    fn on_disconnect(&self, client_id: u64) {

        println!("[{}] disconnected", client_id);

        self.clients.lock().unwrap().remove(&client_id);
    }

    fn on_message(&self, client_id: u64, message: Message) {

        if let Message::Text(text) = message {
            let mut hash_map = self.clients.lock().unwrap();
            let client: &mut Client = hash_map.get_mut(&client_id).unwrap();
            client.process_incoming_message(text);
        }

    }

    pub fn run_broadcast(&self, receiver: &mpsc::Receiver<Packet>) {

        for packet in receiver.iter() {
            self.broadcast(&packet);
        }

    }

    pub fn broadcast(&self, packet: &Packet) {

        println!("send broadcast: {}", packet.get_str(0));

        let text_immutable: String = packet.render_json();
        let hash_map = self.clients.lock().unwrap();
        for (_id, client) in hash_map.iter() {
            let message = Message::Text(text_immutable.clone());
            client.responder.send(message);
        }
    }

}