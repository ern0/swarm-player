#![allow(unused)]

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use simple_websockets::{Message, Responder};
use tinyjson::JsonValue;
use crate::utils::now;
use crate::packet::Packet;

pub struct Client {
    pub id: u64,
    pub responder: Responder,
    pub seen: i64,
    pub epoch: i64,
}

impl Client {

    pub fn new(id: u64, responder: Responder) -> Self {

        return Client {
            id: id,
            responder: responder,
            seen: 0,
            epoch: now(),
        };
    }

    pub fn process_incoming_message(self: &mut Client, text: String) {
        
        self.touch();
        let packet = Packet::from(&text);
        let message_type = packet.get_type();

        if message_type == "CLK_0" {
            self.process_request_clk0(packet);
        }
    }

    fn touch(&mut self) {
        self.seen = now();
    }

    fn process_request_clk0(&self, packet: Packet) {
        
        let skew = packet.get_num(0);
        println!("[{}] clock skew was: {}", self.id, skew);

        sleep(Duration::from_millis(100));
        let clk_server = now();
        println!("[{}] clock sync at: {}", self.id, clk_server);
        sleep(Duration::from_millis(100));

        let packet: Packet = Packet::new_simple_num("CLK_REF", clk_server);
        let json = packet.render_json();        
        self.send_now(&json);
    }

    fn send_now(&self, text: &str) {
        
        let message = Message::Text(text.to_string());
        self.responder.send(message);
    }
}
