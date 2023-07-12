//#![allow(unused)]
#![allow(clippy::needless_return)]

use std::time::SystemTime;
use simple_websockets::{Message, Responder};
use crate::utils::{now_string, systime_to_millis, SharedClientList};
use crate::packet::Packet;

pub struct Client {
    pub clients: SharedClientList,
    pub id: u64,
    pub opt_responder: Option<Responder>,
    pub dirty: bool,
    pub clock_skew: Option<i64>,
    pub audio_lag: Option<i64>,
    pub debug: bool,
}

impl Client {

    pub fn new(clients: SharedClientList, id: u64, opt_responder: Option<Responder>, debug: bool) -> Self {

        return Client {
            clients,
            id,
            opt_responder,
            dirty: true,
            clock_skew: None,
            audio_lag: None,            
            debug,
        };
    }

    pub fn send_packet(&self, packet: &Packet) {

        let text: String = packet.render_json();
        self.send_text(&text);

    }

    pub fn send_text(&self, text: &str) {

        if self.debug {
            println!(
                "[mgr] {}: send({}): {}", 
                now_string(),
                self.id, 
                text,
                );
        }

        let message = Message::Text(text.to_string());
        if let Some(responder) = &self.opt_responder {
            responder.send(message);
        }   
    }

    pub fn process_request_clk0(&self) {
        
        let clk_server = SystemTime::now();
        let clk_millis = systime_to_millis(clk_server);
        let packet: Packet = Packet::new_simple_num("CLK_REF", clk_millis);
        self.send_packet(&packet);
    }

    pub fn process_report_master(&self) {
        
        println!(
            "{} [{}]: master mode", 
            now_string(),
            self.id,
        );
    }

    pub fn process_report_clock_skew(&mut self, packet: &Packet) {

        let value = packet.get_num(0);
        self.clock_skew = Some(value);
        self.dirty = true;

        println!(
            "{} [{}]: clock skew: {} ms", 
            now_string(),
            self.id,
            value,
        );
    }

    pub fn process_report_audio_lag(&mut self, packet: &Packet) {

        let value = packet.get_num(0);
        self.audio_lag = Some(value);
        self.dirty = true;

        println!(
            "{} [{}]: audio lag: {} ms", 
            now_string(),
            self.id,
            value,
        );
    }

    pub fn check_and_clear(&mut self) -> bool {

        if self.dirty {
            self.dirty = false;
            return true;
        } else {
            return false;
        }
    }

    pub fn create_report(&self, master_id: u64) -> Packet {

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

        // TODO: use real mask
        let channel_mask = self.id as i64 & 0x0f;
        packet.set_num(3, channel_mask);

        println!(
            "{} [mgr]: report({}): id={} skew={} lag={} mask={}", 
            now_string(),
            master_id,
            self.id,
            packet.get_str(1),
            packet.get_str(2),
            packet.get_str(3),
        );

        return packet;
    }

}

#[cfg(test)]

mod tests {
    use std::sync::{Arc, RwLock};
    use std::collections::HashMap;
    use simple_websockets::Responder;
    use crate::{client::Client, utils::SharedClient};

    fn create_client(id: u64) -> Client {

        let clients_hash_map: HashMap<u64, SharedClient> = HashMap::new();
        let clients_lock = RwLock::new(clients_hash_map);
        let clients = Arc::new(clients_lock);

        let opt_responder: Option<Responder> = None;

        let client = Client::new(
            clients,
            id,
            opt_responder,
            false,
        );

        return client;
    }

     #[test]
     fn packet_create_type() {
        let client = create_client(11);
        let packet = client.create_report(0);
        let json = packet.render_json();
        assert!(json.contains(r#"type":"REPORT"#));
     }
     #[test]
     fn report_create_empty() {
        let client = create_client(12);
        let packet = client.create_report(0);
        let json = packet.render_json();
        assert!(json.contains(r#"data":[12,"-","-"#));
     }
     #[test]
     fn report_create_skew_set() {
        let mut client = create_client(12);
        client.clock_skew = Some(10);
        let packet = client.create_report(0);
        let json = packet.render_json();
        assert!(json.contains(r#""data":[12,10,"-""#));
     }
     #[test]
     fn report_create_lag_set() {
        let mut client = create_client(16);
        client.audio_lag = Some(3);
        let packet = client.create_report(0);
        let json = packet.render_json();
        assert!(json.contains(r#""data":[16,"-",3,"#));
     }
     #[test]
     fn report_create_both_set() {
        let mut client = create_client(21);
        client.clock_skew = Some(12);
        client.audio_lag = Some(321);
        let packet = client.create_report(0);
        let json = packet.render_json();
        assert!(json.contains(r#""data":[21,12,321,"#));
     }

}
