use crate::utils::{Port, ClientSessionId};
use simple_websockets::Responder;

pub struct ClientManager {

}

impl ClientManager {
	pub fn new() -> Self {

		Self {}
	}

	pub fn on_client_connect(&mut self, client_session_id: ClientSessionId, responder: Responder) {

	}

	pub fn on_client_disconnect(&mut self, client_session_id: ClientSessionId) {

	}

}
