use simple_websockets::{Responder, Message};

use crate::utils::{Port, ClientSessionId};
use crate::packet::Packet;

pub struct ClientManager {

}

impl ClientManager {
	pub fn new() -> Self {

		Self {}
	}

	pub fn on_client_connect(&mut self, client_session_id: ClientSessionId, responder: Responder) {

        println!("connect {}", client_session_id);

        let packet = Packet::new_simple_num("ID", client_session_id as i64);
		let text: String = packet.render_json();
        let message = Message::Text(text.to_string());
        responder.send(message);

	}

	pub fn on_client_disconnect(&mut self, client_session_id: ClientSessionId) {
		println!("disconnect {}", client_session_id);
	}

}
