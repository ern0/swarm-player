use simple_websockets::*;

use crate::utils::{Port, ClientSessionId};
use crate::packet::Packet;

pub struct Logger {
    pub debug_mode: bool,
}

impl Logger {

    pub fn log_webserver_start_success(&self, port: Port) {
        println!("[webserver]: server is up, listening on port {}",
            port
        );
    }

    pub fn log_webserver_start_fail(&self, port: Port, error: &simple_websockets::Error) {
        println!(
            "[webserver]: failed to launch server on port {}: {:?}",
            port, error,
        );
    }

    pub fn log_webserver_message_received(&self, client_session_id: ClientSessionId, text: &String) {

        println!(
            "[session={}]: recv: {}",
            client_session_id,
            text,
            );

    }

    pub fn log_webserver_message_invalid(&self, client_session_id: ClientSessionId) {
        println!(
            "[session={}] non-text message received",
            client_session_id,
        );
    }

    pub fn log_client_connected(&self, client_session_id: ClientSessionId) {
        println!(
            "[session={}] client connected",
            client_session_id,
        );
    }

    pub fn log_client_disconnected(&self, client_session_id: ClientSessionId) {
        println!(
            "[session={}] client disconnected",
            client_session_id,
        );
    }

    pub fn log_packet_send(&self, client_session_id: ClientSessionId, text: &String) {
        println!(
            "[session={}] send {}",
            client_session_id,
            text,
        );
    }

} // impl
