use simple_websockets;
use std::collections::HashMap;

mod client;
use client::Client;

pub fn main() {

    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");

    let mut clients: HashMap<u64, Client> = HashMap::new();

    loop {

        match event_hub.poll_event() {

            simple_websockets::Event::Connect(client_id, responder) => {
                
                println!("A client connected with id #{}", client_id);

                let client = Client { 
                    id: client_id, 
                    responder: responder,
                };
                clients.insert(client.id, client);

            },

            simple_websockets::Event::Disconnect(client_id) => {

                println!("Client #{} disconnected.", client_id);

                clients.remove(&client_id);
                
            },

            simple_websockets::Event::Message(client_id, message) => {

                println!("Received a message from client #{}: {:?}", client_id, message);

                let client = clients.get(&client_id).unwrap();
                let response = client_id.to_string();
                client.responder.send(simple_websockets::Message::Text(response));
            },

        }

    }

}
