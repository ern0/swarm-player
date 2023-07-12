#![allow(unused)]

use std::time::{SystemTime, Duration};
use std::thread::sleep;
use simple_websockets::{Message, Responder};
use crate::utils::{now_string, systime_to_string, systime_to_millis, ClientList};
use crate::packet::Packet;

pub struct Client {
    pub clients: ClientList,
    pub id: u64,
    pub responder: Responder,
    pub seen: SystemTime,
    pub epoch: SystemTime,
}

impl Client {

    pub fn new(clients: ClientList, id: u64, responder: Responder) -> Self {

        return Client {
            clients: clients,
            id: id,
            responder: responder,
            seen: SystemTime::UNIX_EPOCH,
            epoch: SystemTime::now(),
        };
    }

    pub fn process_incoming_message(self: &mut Client, packet: &Packet) {
        
        self.seen = SystemTime::now();

        match packet.get_type().as_str() {
            "CLK_0" => self.process_request_clk0(&packet),
            _ => {},
        }

    }

    fn send(&self, text: &str) {

        println!(
            "[{}] {}: send: {}", 
            self.id, 
            now_string(),
            text,
            );

        let message = Message::Text(text.to_string());
        self.responder.send(message);

    }

    fn process_request_clk0(&self, packet: &Packet) {
        
        let skew = packet.get_num(0);
        let clk_server = SystemTime::now();

        println!(
            "[{}] {}: clock sync, skew was: {}", 
            self.id, 
            systime_to_string(clk_server),
            skew
            );

        let clk_millis = systime_to_millis(clk_server);
        let packet: Packet = Packet::new_simple_num("CLK_REF", clk_millis);
        let json = packet.render_json(clk_server);        
        self.send(&json);
    }

}
