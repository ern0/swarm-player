#![allow(unused)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use std::thread::{sleep, spawn};
use std::sync::{Arc, RwLock, Mutex};
use simple_websockets::{Event, Message, Responder};
use crate::utils::{now_string, millis_to_string};
use crate::utils::{SharedClient, SharedClientList};
use crate::client::Client;
use crate::packet::Packet;

pub struct ClientManager {
    clients: SharedClientList,
    master_client: Arc<RwLock<Option<SharedClient>>>,
    master_client_id: Mutex<Option<u64>>,
    debug: bool,
}

impl ClientManager {

    pub fn new() -> Self {

        let clients_hash_map: HashMap<u64, SharedClient> = HashMap::new();
        let clients_lock = RwLock::new(clients_hash_map);
        let clients = Arc::new(clients_lock);

        let master_client_value: Option<SharedClient> = None;
        let master_client_lock = RwLock::new(master_client_value);
        let master_client = Arc::new(master_client_lock);

        let master_client_id_value: Option<u64> = None;
        let master_client_id = Mutex::new(master_client_id_value);

        return ClientManager {
            clients,
            master_client,
            master_client_id,
            debug: false,
        };
    }

    pub fn start(self) {

        let s0 = Arc::new(self);
        let s1 = s0.clone();
        let s2 = s0.clone();

        spawn(move || s1.run_reporting());
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

        let client = self.create_client(client_id, responder);
        self.report_client_creation(client_id, &client);
        self.add_to_clients(client_id, client);
    }

    fn on_disconnect(&self, client_id: u64) {

        println!(
            "[{}] {}: disconnected", 
            client_id,
            now_string(),
            );
      
        self.remove_from_clients(client_id);
        self.clear_master_id_on_match(client_id);
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

    fn create_client(&self, client_id: u64, responder: Responder) -> Client {

        let shared_clients = self.clients.clone();
        let client = Client::new(shared_clients, client_id, responder, self.debug);

        return client;
    }

    fn report_client_creation(&self, client_id: u64, client: &Client) {

        let packet = Packet::new_simple_num("ID", client_id as i64);
        client.send_packet(&packet);

    }

    fn add_to_clients(&self, client_id: u64, client: Client) {

        let shared_client = Arc::new(RwLock::new((client)));
        let mut hash_map = self.clients.write().unwrap();
        hash_map.insert(client_id, shared_client);

    }

    fn remove_from_clients(&self, client_id: u64) {

        let mut hash_map = self.clients.write().unwrap();
        hash_map.remove(&client_id);

    }

    fn set_master_client(&self, shared_client: &SharedClient) {

        let mut value = self.master_client.write().unwrap();
        *value = Some(shared_client.clone());

    }

    fn set_master_client_id(&self, client_id: u64) {

        let mut opt: &Option<u64> = &self.master_client_id.lock().unwrap();
        opt = &Some(client_id);

    }

    fn clear_master_id_on_match(&self, client_id: u64) {

        let mut opt: &Option<u64> = &self.master_client_id.lock().unwrap();

        let master_id = match opt {
            Some(value) => value,
            None => return,
        };

        if &client_id == master_id {
            opt = &None;    
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

            "MASTER" => {
                self.set_master_client(shared_client);
                self.set_master_client_id(client_id);

                let client = shared_client.read().unwrap();
                client.process_report_master();
            },

            "CLOCK_SKEW" => {
                let mut client = shared_client.write().unwrap();
                client.process_report_clock_skew(&packet);
            },

            "AUDIO_LAG" => {
                let mut client = shared_client.write().unwrap();
                client.process_report_audio_lag(&packet);
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

    fn run_reporting(&self) {

        loop {
            self.wait_for_master_client();
            self.report_to_master_client();
        }
    }    

    fn wait_for_master_client(&self) {

        loop {

            sleep(Duration::from_secs(1));

            let lock: &RwLock<Option<SharedClient>> = &self.master_client;
            let opt: &Option<SharedClient> = &lock.read().unwrap();
            let shared_client = match opt {
                Some(_) => return,
                None => continue,
            };

        }
    }

    fn report_to_master_client(&self) {

        // TODO: report actual set or something, sleep some
        let client_ids = self.get_client_ids_snapshot();

        loop {

            let lock: &RwLock<Option<SharedClient>> = &self.master_client;
            let opt: &Option<SharedClient> = &lock.read().unwrap();
            let shared_master_client = match opt {
                Some(shc) => shc,
                None => return,
            };

            for client_id in &client_ids {

                let hash_map = self.clients.read().unwrap();
                if !hash_map.contains_key(&client_id) {
                    continue;
                }

                let shared_client = hash_map.get(&client_id).unwrap();
                let client = shared_client.read().unwrap();
                let packet = client.create_report();
                let master_client = shared_master_client.read().unwrap();
                master_client.send_packet(&packet);
            }
        }
    }

    fn get_client_ids_snapshot(&self) -> Vec<u64> {

        let mut client_ids = Vec::new();

        let hash_map = self.clients.read().unwrap();
        for (id, _shared_client) in hash_map.iter() {
            client_ids.push(*id);
        }

        return client_ids;
    }

}
