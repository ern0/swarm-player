//#![allow(unused)]

use std::time::SystemTime;
use simple_websockets::{Message, Responder};
use crate::utils::{now_string, systime_to_millis, SharedClientList};
use crate::packet::Packet;

pub struct Client {
    pub clients: SharedClientList,
    pub id: u64,
    pub lag: i64,
    pub responder: Responder,
    pub debug: bool,
    pub epoch: SystemTime,
}

impl Client {

    pub fn new(clients: SharedClientList, id: u64, responder: Responder, debug: bool) -> Self {

        return Client {
            clients: clients,
            id: id,
            lag: 0,
            responder: responder,
            debug: debug,
            epoch: SystemTime::now(),
        };
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

    pub fn process_request_clk0(&self) {
        
        let clk_server = SystemTime::now();
        let clk_millis = systime_to_millis(clk_server);
        let packet: Packet = Packet::new_simple_num("CLK_REF", clk_millis);
        self.send_packet(&packet);
    }

    pub fn process_report_admin(&self) {
        
        println!(
            "[{}] {}: admin mode", 
            self.id,
            now_string(),
            );
    }

    pub fn process_report_audio(&mut self, packet: &Packet) {

        self.lag = packet.get_num(0);

        println!(
            "[{}] {}: audio lag: {} ms", 
            self.id,
            now_string(),
            self.lag,
            );
    }

}
