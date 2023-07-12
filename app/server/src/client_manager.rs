#![allow(unused)]

use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use simple_websockets::{ Event, EventHub, Responder, Message };

use crate::client::Client;

pub struct ClientManager {
	event_hub: EventHub,
	clients: Mutex<HashMap<u64, Client>>,
}

impl ClientManager {

	pub fn new(event_hub: EventHub) -> Self {
		return ClientManager {
			event_hub: event_hub,
			clients: Mutex::new(HashMap::new()),
		}
	}

	pub fn run(&self) {

        loop { 
            match self.event_hub.poll_event() {

                Event::Connect(client_id, responder) => {
                    self.on_connect(responder, client_id);                    
                },

                Event::Disconnect(client_id) => {
                    self.on_disconnect(client_id);                
                },

                Event::Message(client_id, message) => {
                    self.on_message(client_id, message);
                },

            }
        }
    }

    fn on_connect(&self, responder: Responder, client_id: u64) {

        println!("connected, id={}", client_id);

        let client = Client::new(client_id, responder);
        self.clients.lock().unwrap().insert(client.id, client);

    }

    fn on_disconnect(&self, client_id: u64) {

        println!("disconnected, id={}", client_id);

        self.clients.lock().unwrap().remove(&client_id);

    }

    fn on_message(&self, client_id: u64, message: Message) {

        if let Message::Text(text) = message {
            
            println!("received, client={} message={}", client_id, text);

            let mut hash_map = self.clients.lock().unwrap();
            let client: &mut Client = hash_map.get_mut(&client_id).unwrap();
            client.process_incoming_message(text);
        }

    }

    pub fn broadcast(&self) {

        let hash_map = self.clients.lock().unwrap();
        let size = hash_map.len();

        //println!("Sending broadcast: {}", size);

        for (_id, client) in hash_map.iter() {
       		client.responder.send(Message::Text(size.to_string()));
        }
    }

}