use simple_websockets::{ Responder };

pub struct Client {
	pub id: u64,
	pub responder: Responder,
}

impl Client {

	pub fn process_incoming_message(self: &Client, _: simple_websockets::Message) {		


	}

}

