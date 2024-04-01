#![allow(unused)]

mod logger;
mod utils;
mod packet;
mod webserver;
mod client_manager;

use std::sync::Arc;
use std::time::Duration;
use std::thread::sleep;

use crate::logger::Logger;
use crate::webserver::WebServer;
use crate::client_manager::ClientManager;

pub fn main() {

    let listen_port = 8080;
    let debug_mode = true;

    let logger = Arc::new(Logger { debug_mode });
    let client_manager = ClientManager { logger: logger.clone() };
	let webserver = WebServer { listen_port, logger, client_manager };
	webserver.start();

    loop { sleep(Duration::from_secs(1)); }
}
