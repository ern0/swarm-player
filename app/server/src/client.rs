#![allow(unused)]

use simple_websockets::{ Message, Responder };

pub struct Client {
	pub id: u64,
	//pub responder: Responder,
}

impl Client {

	pub fn process_incoming_message(self: &Client, _: simple_websockets::Message) {		

        let response = self.id.to_string();
		//self.responder.send(Message::Text(response));

	}

}
