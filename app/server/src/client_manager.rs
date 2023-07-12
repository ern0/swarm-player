#![allow(unused)]

use std::collections::HashMap;
use std::time::Duration;
use std::thread::{sleep, spawn};
use std::sync::{Arc, RwLock};
use simple_websockets::{Event, Message, Responder};
use crate::utils::ClientList;
use crate::client::Client;
use crate::packet::Packet;

pub struct ClientManager {
    clients: ClientList,
}

impl ClientManager {

    pub fn new() -> Self {

        return ClientManager {
            clients: Arc::new(RwLock::new(HashMap::new())),
        };
    }

    pub fn start(self) {

        let s0 = Arc::new(self);
        let s1 = s0.clone();
        let s2 = s0.clone();

        spawn(move || s1.run_display_counter());
        spawn(move || s2.run_event_hub());
    }

    fn broadcast(&self, packet: &Packet) {

        let text_immutable: String = packet.render_json();
        println!("<mgr> broadcast: {}", text_immutable);

        let hash_map = self.clients.write().unwrap();
        for (_id, client) in hash_map.iter() {
            let message = Message::Text(text_immutable.clone());
            client.responder.send(message);
        }
    }

    pub fn run_event_hub(&self) {

        let event_hub = simple_websockets::launch(8080)
            .expect("failed to listen on port 8080");
        println!("server is up");

        loop {
            match event_hub.poll_event() {
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

        let arc = self.clients.clone();
        let client = Client::new(arc, client_id, responder);
        self.clients.write().unwrap().insert(client.id, client);

    }

    fn on_disconnect(&self, client_id: u64) {

        println!("[{}] disconnected", client_id);

        self.clients.write().unwrap().remove(&client_id);
    }

    fn on_message(&self, client_id: u64, message: Message) {

        if let Message::Text(text) = message {

            println!("[{}] message: {}", client_id, text);

            let packet = Packet::from(&text);
            match packet.get_type().as_str() {

                "COLOR" => {
                    self.process_request_color(&packet);
                },

                _ => {
                    let mut hash_map = self.clients.write().unwrap();
                    let client: &mut Client = hash_map.get_mut(&client_id).unwrap();
                    client.process_incoming_message(&packet);
                },

            }
        }

    }

    fn process_request_color(&self, packet: &Packet) {
        let _color = packet.get_str(0);
        self.broadcast(packet);
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