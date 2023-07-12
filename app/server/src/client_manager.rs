#![allow(unused)]

use std::collections::HashMap;
use std::time::Duration;
use std::thread::{sleep, spawn};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use simple_websockets::{Event, EventHub, Message, Responder};
use crate::utils::now;
use crate::client::Client;
use crate::packet::Packet;

pub struct ClientManager {
    event_hub: EventHub,
    clients: Mutex<HashMap<u64, Client>>,
}

impl ClientManager {

    pub fn new(event_hub: EventHub) -> Self {

        return ClientManager {
            event_hub: event_hub,
            clients: Mutex::new(HashMap::new()),
        };
    }

    pub fn start(self) {

        let s0 = Arc::new(self);
        let s1 = s0.clone();
        let s2 = s0.clone();

        spawn(move || s1.run_event_hub());
        spawn(move || s2.run_display_counter());

    }

    fn broadcast(&self, packet: &Packet) {

        println!("send broadcast: {}", packet.get_str(0));

        let text_immutable: String = packet.render_json();
        let hash_map = self.clients.lock().unwrap();
        for (_id, client) in hash_map.iter() {
            let message = Message::Text(text_immutable.clone());
            client.responder.send(message);
        }
    }

    pub fn run_event_hub(&self) {

        loop {
            match self.event_hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    self.on_connect(client_id, responder);
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

    fn on_connect(&self, client_id: u64, responder: Responder) {

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

    fn run_display_counter(&self) {

        let mut counter = 0;
        loop {        
            let mut packet = Packet::new_simple_num("DISPLAY", counter);
            packet.set_num(0, counter);
            self.broadcast(&packet);
            counter += 1;

            sleep(Duration::from_secs(1));
        }

    }    

}