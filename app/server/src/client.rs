//#![allow(unused)]

use std::time::SystemTime;
use simple_websockets::{Message, Responder};
use crate::utils::{now_string, systime_to_millis, ClientList};
use crate::packet::Packet;

pub struct Client {
    pub clients: ClientList,
    pub id: u64,
    pub control_station: bool,
    pub lag: i64,
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
            control_station: false,
            lag: 0,
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
            "CTRL" => self.process_report_ctrl(),
            "AUDIO" => self.process_report_audio(&packet),
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

    fn process_report_ctrl(&mut self) {
        
        self.control_station = true;

        println!(
            "[{}] {}: control station selected", 
            self.id,
            now_string(),
            );
    }

    fn process_report_audio(&mut self, packet: &Packet) {

        self.lag = packet.get_num(0);

        println!(
            "[{}] {}: audio lag: {} ms", 
            self.id,
            now_string(),
            self.lag,
            );
    }

}
