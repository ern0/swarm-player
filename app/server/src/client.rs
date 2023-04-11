//#![allow(unused)]

use std::time::SystemTime;
use simple_websockets::{Message, Responder};
use crate::utils::{now_string, systime_to_millis, SharedClientList};
use crate::packet::Packet;

pub struct Client {
    pub clients: SharedClientList,
    pub id: u64,
    pub clock_skew: Option<i64>,
    pub audio_lag: Option<i64>,
    pub responder: Responder,
    pub debug: bool,
}

impl Client {

    pub fn new(clients: SharedClientList, id: u64, responder: Responder, debug: bool) -> Self {

        return Client {
            clients: clients,
            id: id,
            clock_skew: None,
            audio_lag: None,            
            responder: responder,
            debug: debug,
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

    pub fn process_report_master(&self) {
        
        println!(
            "[{}] {}: master mode", 
            self.id,
            now_string(),
            );
    }

    pub fn process_report_clock_skew(&mut self, packet: &Packet) {

        let value = packet.get_num(0);
        self.clock_skew = Some(value);

        println!(
            "[{}] {}: clock skew: {} ms", 
            self.id,
            now_string(),
            value,
            );
    }

    pub fn process_report_audio_lag(&mut self, packet: &Packet) {

        let value = packet.get_num(0);
        self.audio_lag = Some(value);

        println!(
            "[{}] {}: audio lag: {} ms", 
            self.id,
            now_string(),
            value,
            );
    }

    pub fn report(&self) {

        let mut packet = Packet::new();
        packet.set_type("REPORT");

        packet.set_num(0, self.id as i64);

        match self.clock_skew {
            Some(value) => packet.set_num(1, value),
            None => packet.set_str(1, "-"),
        };

        match self.audio_lag {
            Some(value) => packet.set_num(2, value),
            None => packet.set_str(2, "-"),
        };

        let channel_mask = 0x00;
        packet.set_num(3, channel_mask);
        
        self.send_packet(&packet);
    }

}
