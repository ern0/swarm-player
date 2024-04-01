use std::sync::Arc;
use simple_websockets::{Responder, Message};

use crate::logger::Logger;
use crate::utils::{Port, ClientSessionId};
use crate::packet::{Packet, PacketInt};

pub struct ClientManager {
    pub logger: Arc<Logger>,

}

impl ClientManager {

    pub fn send_packet(
        &self,
        packet: Packet,
        client_session_id: ClientSessionId,
        responder: Responder,
    ) {
        let text: String = packet.render_json();
        self.logger.log_packet_send(client_session_id, &text);
        let message = Message::Text(text.to_string());
        responder.send(message);
    }

    pub fn on_client_connect(&mut self, client_session_id: ClientSessionId, responder: Responder) {
        self.logger.log_client_connected(client_session_id);

        let packet = Packet::new_simple_num("ID", client_session_id as PacketInt);
        self.send_packet(packet, client_session_id, responder);
    }

    pub fn on_client_disconnect(&mut self, client_session_id: ClientSessionId) {
        self.logger.log_client_disconnected(client_session_id);
    }
}
