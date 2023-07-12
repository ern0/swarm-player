use simple_websockets::{Event, Responder};
use std::collections::HashMap;

pub fn main() {
    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    loop {

        match event_hub.poll_event() {

            Event::Connect(client_id, responder) => {
                println!("A client connected with id #{}", client_id);
                clients.insert(client_id, responder);
            },

            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);
                clients.remove(&client_id);
            },

            Event::Message(client_id, message) => {
                println!("Received a message from client #{}: {:?}", client_id, message);
                let responder = clients.get(&client_id).unwrap();
                let response = client_id.to_string();
                responder.send(simple_websockets::Message::Text(response));
            },
        }

    } 
}