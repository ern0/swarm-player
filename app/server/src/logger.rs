use simple_websockets::*;

use crate::utils::{Port};

pub struct Logger {
    debug_mode: bool,
}

impl Logger {
    pub fn new(debug_mode: bool) -> Self {
        Self {
            debug_mode
        }
    }

    pub fn log_webserver_okay(&self, port: Port) {
        println!("[mgr]: server is up, listening on port {}",
            port
        );
    }

    pub fn log_webserver_fail(&self, port: Port, error: &simple_websockets::Error) {
        println!(
            "[mgr]: failed to launch server on port {}: {:?}",
            port, error,
        );
    }
}
