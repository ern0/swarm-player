#![allow(unused)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread::spawn;
use std::sync::{Arc, RwLock};
use simple_websockets::{Event, Message, Responder};
use crate::utils::{now_string, millis_to_string};
use crate::utils::SharedClientList;
use crate::reporting::Reporting;
use crate::channel_manager::ChannelManager;
use crate::client::Client;
use crate::packet::Packet;

pub struct ClientManager {
    clients: SharedClientList,
    reporting: Option<Arc<Reporting>>,
    channel_manager: Option<Arc<ChannelManager>>,
    debug: bool,
}

impl ClientManager {

    pub fn new(clients: SharedClientList) -> Self {

        return ClientManager {
            clients,
            reporting: None,
            channel_manager: None,
            debug: false,
        };
    }

    pub fn start(mut self) {

        let naked_reporting = Reporting::new(self.clients.clone());
        self.reporting = Some(Arc::new(naked_reporting));

        let naked_channel_manager = ChannelManager::new(self.clients.clone());
        self.channel_manager = Some(Arc::new(naked_channel_manager));

        spawn(move || self.run_event_hub());
    }

    fn broadcast(&self, packet: &Packet) {

        let text_immutable: String = packet.render_json();

        if self.debug {
            println!(
                "{} [mgr]: broadcast: {}", 
                now_string(),
                text_immutable,
            );
        }

        let hash_map = self.clients.read().unwrap();
        for (_id, shared_client) in hash_map.iter() {
            let message = Message::Text(text_immutable.clone());
            let client = shared_client.write().unwrap();
            if let Some(responder) = &client.opt_responder {
                responder.send(message);
            }
        }
        
    }

    pub fn run_event_hub(&self) {

        let event_hub = simple_websockets::launch(8080)
            .expect("failed to listen on port 8080");

        println!(
            "{} [mgr]: server is up",
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
            "{} [{}]: connected", 
            now_string(),
            client_id,
        );

        let client = self.create_client(client_id, responder);
        self.send_id_to_client(client_id, &client);
        let naked_reporting = self.reporting.as_ref().unwrap();
        naked_reporting.report_client_creation(client_id);
        self.add_to_clients(client_id, client);
    }

    fn on_disconnect(&self, client_id: u64) {

        println!(
            "{} [{}]: disconnected", 
            now_string(),
            client_id,
            );
      
        let naked_reporting = self.reporting.as_ref().unwrap();
        naked_reporting.report_client_destruction(client_id);
        self.remove_from_clients(client_id);
        naked_reporting.clear_master_on_match(client_id);
    }

    fn on_message(&self, client_id: u64, message: Message) {

        if let Message::Text(text) = message {

            if self.debug {
                println!(
                    "{} [{}]: recv: {}", 
                    now_string(),
                    client_id,
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
        let opt_responder = Some(responder);
        let client = Client::new(shared_clients, client_id, opt_responder, self.debug);

        return client;
    }

    fn send_id_to_client(&self, client_id: u64, client: &Client) {

        let packet = Packet::new_simple_num("ID", client_id as i64);
        client.send_packet(&packet);

    }

    fn add_to_clients(&self, client_id: u64, client: Client) {

        let shared_client = Arc::new(RwLock::new(client));
        let mut hash_map = self.clients.write().unwrap();
        hash_map.insert(client_id, shared_client);

    }

    fn remove_from_clients(&self, client_id: u64) {

        let mut hash_map = self.clients.write().unwrap();
        hash_map.remove(&client_id);

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
                let naked_reporting = self.reporting.as_ref().unwrap();
                naked_reporting.set_master_client(shared_client);
                naked_reporting.set_master_client_id(client_id);

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
            if packet.get_bool(1) { "!" } else { "" }
        };
        let stamp_millis = packet.get_num(2);
        let stamp_string = millis_to_string(stamp_millis);

        println!(
            "{}{} [{}]: {}",
            stamp_string, 
            delay_mark, 
            client_id, 
            message,
            );       
        if let Err(e) = writeln!(
            file, 
            "{}{} [{}]: {}", 
            stamp_string,
            delay_mark,
            client_id, 
            message,
            ) {
            eprintln!("*** LOG error: {}", e);
        }
    }

}
