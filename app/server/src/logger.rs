use simple_websockets::*;

use crate::utils::{Port, ClientSessionId};

pub struct Logger {
    debug_mode: bool,
}

impl Logger {
    pub fn new(debug_mode: bool) -> Self {
        Self {
            debug_mode
        }
    }

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

}
