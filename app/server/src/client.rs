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

    pub fn process_incoming_message(self: &mut Client, text: String) {
        
        self.seen = now();
        let packet = Packet::from(&text);
        println!("[{}] message: {}", self.id, text);

        match packet.get_type().as_str() {
            "CLK_0" => self.process_request_clk0(&packet),
            "COLOR" => self.process_request_color(&packet),
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

        sleep(Duration::from_millis(100));
        let clk_server = now();
        println!("[{}] clock sync at: {}", self.id, clk_server);
        sleep(Duration::from_millis(100));

        let packet: Packet = Packet::new_simple_num("CLK_REF", clk_server);
        let json = packet.render_json();        
        self.send_now(&json);
    }

    fn process_request_color(&self, packet: &Packet) {

        let color = packet.get_str(0);
        println!("[{}] color: {}", self.id, color);

        self.broadcast(packet);
    }

    fn broadcast(&self, packet: &Packet) {

        println!("[{}] send broadcast: {}", self.id, packet.get_str(0));

        let text_immutable: String = packet.render_json();
        println!("---------------c0");
        let hash_map = self.clients.lock().unwrap();
        println!("---------------c1");
        for (_id, client) in hash_map.iter() {
            let message = Message::Text(text_immutable.clone());
            client.responder.send(message);
        }
    }

}
