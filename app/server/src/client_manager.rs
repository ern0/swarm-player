#![allow(unused)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread::spawn;
use std::sync::{Arc, RwLock};
use simple_websockets::{Event, Message, Responder};
use crate::utils::{now_string, millis_to_string};
use crate::utils::{ClientId, SharedClientList};
use crate::reporting::Reporting;
use crate::channel_manager::ChannelManager;
use crate::client::Client;
use crate::packet::Packet;

pub struct ClientManager {
    client_list: SharedClientList,
    reporting: Reporting,
    channel_manager: ChannelManager,
    debug: bool,
}

impl ClientManager {

    pub fn new(client_list: SharedClientList, reporting: Reporting) -> Self {

        return ClientManager {
            client_list: client_list.clone(),
            reporting,
            channel_manager: ChannelManager::new(client_list),
            debug: false,
        };
    }

    pub fn start(mut self) {
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

        let hash_map = self.client_list.read().unwrap();
        for (_id, shared_client) in hash_map.iter() {
            let message = Message::Text(text_immutable.clone());
            let client = shared_client.write().unwrap();
            if let Some(responder) = &client.opt_responder {
                responder.send(message);
            }
        }

    }

    pub fn run_event_hub(&mut self) {

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

    fn on_connect(&mut self, client_id: ClientId, responder: Responder) {

        println!(
            "{} [{}]: connected",
            now_string(),
            client_id,
        );

        let client = self.create_client(client_id, responder);
        self.send_id_to_client(client_id, &client);
        self.add_to_client_list(client_id, client);

        self.channel_manager.report_client_creation(client_id);
        self.reporting.report_client_creation(client_id);
    }

    fn on_disconnect(&mut self, client_id: ClientId) {

        println!(
            "{} [{}]: disconnected",
            now_string(),
            client_id,
            );

        self.reporting.report_client_destruction(client_id);
        self.remove_from_client_list(client_id);
        self.reporting.clear_admin_on_match(client_id);
        self.channel_manager.report_client_destruction(client_id);
    }

    fn on_message(&self, client_id: ClientId, message: Message) {

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

    fn create_client(&self, client_id: ClientId, responder: Responder) -> Client {

        let shared_client_list = self.client_list.clone();
        let opt_responder = Some(responder);
        let client = Client::new(shared_client_list, client_id, opt_responder, self.debug);

        return client;
    }

    fn send_id_to_client(&self, client_id: ClientId, client: &Client) {

        let packet = Packet::new_simple_num("ID", client_id as i64);
        client.send_packet(&packet);

    }

    fn add_to_client_list(&self, client_id: ClientId, client: Client) {

        let shared_client = Arc::new(RwLock::new(client));
        let mut hash_map = self.client_list.write().unwrap();
        hash_map.insert(client_id, shared_client);

    }

    fn remove_from_client_list(&self, client_id: ClientId) {

        let mut hash_map = self.client_list.write().unwrap();
        hash_map.remove(&client_id);

    }


    fn process_incoming_message(&self, client_id: ClientId, packet: &Packet) {

        let hash_map = self.client_list.read().unwrap();
        let shared_client = hash_map.get(&client_id).unwrap();

        match packet.get_type().as_str() {

            "CLK_0" => {
                let client = shared_client.read().unwrap();
                client.process_request_clk0();
            },

            "ADMIN" => {
                self.reporting.set_admin_client(shared_client);
                self.reporting.set_admin_client_id(client_id);

                let client = shared_client.read().unwrap();
                client.process_report_admin();
                self.reporting.sync_client_list();
            },

            "CLOCK_SKEW" => {
                let mut client = shared_client.write().unwrap();
                client.process_report_clock_skew(packet);
            },

            "AUDIO_LAG" => {
                let mut client = shared_client.write().unwrap();
                client.process_report_audio_lag(packet);
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

    fn process_request_log(&self, client_id: ClientId, packet: &Packet) {

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
