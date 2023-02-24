use simple_websockets;

pub struct Client {
	pub id: u64,
	pub responder: simple_websockets::Responder,
}

