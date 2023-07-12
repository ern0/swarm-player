#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;
use simple_websockets::{Message, Responder};
use crate::utils::{now, ClientList};
use crate::packet::Packet;

pub struct Client {
    pub clients: ClientList,
    pub id: u64,
    pub responder: Responder,
    pub seen: i64,
    pub epoch: i64,
}

impl Client {

    pub fn new(clients: ClientList, id: u64, responder: Responder) -> Self {

        return Client {
            clients: clients,
            id: id,
            responder: responder,
            seen: 0,
            epoch: now(),
        };
    }

    pub fn process_incoming_message(self: &mut Client, packet: &Packet) {
        
        self.seen = now();

        match packet.get_type().as_str() {
            "CLK_0" => self.process_request_clk0(&packet),
            _ => {},
        }
    }

    fn send_now(&self, text: &str) {
        
        println!("[{}] send: {}", self.id, text);
        let message = Message::Text(text.to_string());
        self.responder.send(message);
    }

    fn process_request_clk0(&self, packet: &Packet) {
        
        let skew = packet.get_num(0);
        println!("[{}] clock skew was: {}", self.id, skew);

        let clk_server = now();
        println!("[{}] clock sync at: {}", self.id, clk_server);

        let packet: Packet = Packet::new_simple_num("CLK_REF", clk_server);
        let json = packet.render_json(clk_server);        
        self.send_now(&json);
    }

}
