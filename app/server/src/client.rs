#![allow(unused)]

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use simple_websockets::{Message, Responder};
use tinyjson::JsonValue;
use crate::utils::{json_add_key, json_add_quoted, now};
use crate::packet::{ Packet };

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
        
        let _clk0 = packet.get_num(0);

        sleep(Duration::from_millis(100));
        let clk_server = now();
        println!("report for clock sync: {}", clk_server - 1677710000000);
        sleep(Duration::from_millis(100));

        self.send_response_int(String::from("CLK_REF"), clk_server);
    }

    fn send_response_int(&self, command: String, value: i64) {

        let mut response: String = String::from("{");
        json_add_key(&mut response, "type");
        json_add_quoted(&mut response, &command);
        response.push_str(",");
        json_add_key(&mut response, "data");
        response.push_str("[");
        response.push_str(&value.to_string());
        response.push_str("]");
        response.push_str("}");

        self.send_now(response);
    }

    fn send_now(&self, text: String) {
        
        let message = Message::Text(text);
        self.responder.send(message);
    }
}
