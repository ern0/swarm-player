#![allow(unused)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use std::thread::{sleep, spawn};
use std::sync::{Arc, RwLock};
use simple_websockets::{Event, Message, Responder};
use crate::utils::{now_string, millis_to_string};
use crate::utils::{SharedClient, SharedClientList};
use crate::client::Client;
use crate::packet::Packet;

pub struct ClientManager {
    clients: SharedClientList,
    ////master_client: SharedClient,
    debug: bool,
}

impl ClientManager {

    pub fn new() -> Self {

        let hash_map: HashMap<u64, SharedClient> = HashMap::new();
        let lock = RwLock::new(hash_map);
        let clients = Arc::new(lock);

        return ClientManager {
            debug: false,
            clients,
        };
    }

    pub fn start(self) {

        let s0 = Arc::new(self);
        let s1 = s0.clone();
        let s2 = s0.clone();

        spawn(move || s1.run_display_counter());
        spawn(move || s2.run_event_hub());
    }

    fn broadcast(&self, packet: &Packet) {

        let text_immutable: String = packet.render_json();

        if self.debug {
            println!(
                "[mgr] {}: broadcast: {}", 
                now_string(),
                text_immutable,
                );
        }

        let hash_map = self.clients.read().unwrap();
        for (_id, shared_client) in hash_map.iter() {
            let message = Message::Text(text_immutable.clone());
            let client = shared_client.write().unwrap();
            client.responder.send(message);
        }
        
    }

    pub fn run_event_hub(&self) {

        let event_hub = simple_websockets::launch(8080)
            .expect("failed to listen on port 8080");

        println!(
            "[mgr] {}: server is up",
            now_string(),
            );

        loop {
            match event_hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    self.on_connect(client_id, responder);
                }
                Event::Disconnect(client_id) => {
                    self.on_disconnect(client_id);
                }
                Event::Message(client_id, message) => {
                    self.on_message(client_id, message);
                }
            }
        }
    }

    fn on_connect(&self, client_id: u64, responder: Responder) {

        println!(
            "[{}] {}: connected", 
            client_id,
            now_string(),
            );

        let shared_clients = self.clients.clone();
        let client = Client::new(shared_clients, client_id, responder, self.debug);

        let packet = Packet::new_simple_num("ID", client_id as i64);
        client.send_packet(&packet);

        let shared_client = Arc::new(RwLock::new((client)));
        let mut hash_map = self.clients.write().unwrap();
        hash_map.insert(client_id, shared_client);

    }

    fn on_disconnect(&self, client_id: u64) {

        println!(
            "[{}] {}: disconnected", 
            client_id,
            now_string(),
            );

        let mut hash_map = self.clients.write().unwrap();
        hash_map.remove(&client_id);
    }

    fn on_message(&self, client_id: u64, message: Message) {

        if let Message::Text(text) = message {

            if self.debug {
                println!(
                    "[{}] {}: recv: {}", 
                    client_id,
                    now_string(),
                    text,
                    );
            }

            let packet = Packet::from(&text);
            match packet.get_type().as_str() {

                "LOG" => {
                    self.process_request_log(client_id, &packet);
                },

                "RELOAD" => {
                    self.process_request_reload(&packet);
                },

                "COLOR" => {
                    self.process_request_color(&packet);
                },

                _ => {
                    self.process_incoming_message(client_id, &packet);
                },

            }
        }

    }

    fn process_incoming_message(&self, client_id: u64, packet: &Packet) {

        let hash_map = self.clients.read().unwrap();
        let shared_client = hash_map.get(&client_id).unwrap();

        match packet.get_type().as_str() {
            "CLK_0" => {
                let client = shared_client.read().unwrap();
                client.process_request_clk0();
            },            
            "ADMIN" => {
                let mut client = shared_client.write().unwrap();
                client.process_report_admin();
            },
            "AUDIO" => {
                let mut client = shared_client.write().unwrap();
                client.process_report_audio(&packet);
            },
            _ => {},
        }

    }

    fn process_request_reload(&self, packet: &Packet) {
        self.broadcast(packet);
    }

    fn process_request_color(&self, packet: &Packet) {

        let _color = packet.get_str(0);
        self.broadcast(packet);
    }

    fn process_request_log(&self, client_id: u64, packet: &Packet) {

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/log.txt")    // TODO: better path
            .unwrap();

        let message = packet.get_str(0);
        let delay_mark = {
            if packet.get_bool(1) { "!: >" } else { ": >" }
        };
        let stamp_millis = packet.get_num(2);
        let stamp_string = millis_to_string(stamp_millis);

        println!(
            "[{}] {}{} {}",
            client_id, 
            stamp_string, 
            delay_mark, 
            message,
            );       
        if let Err(e) = writeln!(
            file, 
            "[{}] {}{} {}", 
            client_id, 
            stamp_string,
            delay_mark,
            message,
            ) {
            eprintln!("*** LOG error: {}", e);
        }
    }

    fn run_display_counter(&self) {

        let mut packet = Packet::new();
        packet.set_type("DISPLAY");
        packet.set_str(0, "counter");

        sleep(Duration::from_secs(1));

        let mut counter = 0;
        loop {        
            packet.set_num(1, counter);
            self.broadcast(&packet);
            counter += 1;

            sleep(Duration::from_secs(100));  // TODO: revisit this function
        }

    }    

}
