use simple_websockets::Message;

pub struct Client {
	pub id: u64,
	pub responder: simple_websockets::Responder,
}

impl Client {

	pub fn process_incoming_message(self: &Client, _: simple_websockets::Message) {		

        let response = self.id.to_string();
		self.responder.send(Message::Text(response));

	}

}
