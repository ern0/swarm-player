#![allow(unused)]

use std::collections::HashMap;
use simple_websockets::{ Event, EventHub, Responder, Message };

use crate::client::Client;

pub struct ClientManager {
	event_hub: EventHub,
	clients: HashMap<u64, Client>,
}

impl ClientManager {

	pub fn new(event_hub: EventHub) -> Self {
		return ClientManager {
			event_hub: event_hub,
			clients: HashMap::new(),
		}
	}

	pub fn run(mut self: ClientManager) {

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

    fn on_connect(self: &mut ClientManager, responder: Responder, client_id: u64) {

        println!("A client connected with id #{}", client_id);

        let client = Client { 
            id: client_id, 
            responder: responder,
        };
        self.clients.insert(client.id, client);

    }

    fn on_disconnect(self: &mut ClientManager, client_id: u64) {

        println!("Client #{} disconnected.", client_id);
        self.clients.remove(&client_id);

    }

    fn on_message(self: &ClientManager, client_id: u64, message: Message) {

        println!("Received a message from client #{}: {:?}", client_id, message);

        let client = self.clients.get(&client_id).unwrap();
        client.process_incoming_message(message);

    }

}