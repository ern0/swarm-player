//#![allow(unused)]

use std::time::SystemTime;
use simple_websockets::{Message, Responder};
use crate::utils::{now_string, systime_to_millis, ClientList};
use crate::packet::Packet;

pub struct Client {
    pub clients: ClientList,
    pub id: u64,
    pub responder: Responder,
    pub debug: bool,
    pub seen: SystemTime,
    pub epoch: SystemTime,
}

impl Client {

    pub fn new(clients: ClientList, id: u64, responder: Responder, debug: bool) -> Self {

        return Client {
            clients: clients,
            id: id,
            responder: responder,
            debug: debug,
            seen: SystemTime::UNIX_EPOCH,
            epoch: SystemTime::now(),
        };
    }

    pub fn process_incoming_message(self: &mut Client, packet: &Packet) {
        
        self.seen = SystemTime::now();

        match packet.get_type().as_str() {
            "CLK_0" => self.process_request_clk0(),
            _ => {},
        }

    }

    pub fn send_packet(&self, packet: &Packet) {

        let text: String = packet.render_json();
        self.send_text(&text);

    }

    pub fn send_text(&self, text: &str) {

        if self.debug {
            println!(
                "[{}] {}: send: {}", 
                self.id, 
                now_string(),
                text,
                );
        }

        let message = Message::Text(text.to_string());
        self.responder.send(message);

    }

    fn process_request_clk0(&self) {
        
        let clk_server = SystemTime::now();
        let clk_millis = systime_to_millis(clk_server);
        let packet: Packet = Packet::new_simple_num("CLK_REF", clk_millis);
        self.send_packet(&packet);
    }

}
