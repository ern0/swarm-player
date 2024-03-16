#![allow(unused)]

mod utils;
mod logger;
mod webserver;
mod client_manager;

use crate::logger::Logger;
use crate::webserver::WebServer;
use crate::client_manager::ClientManager;

pub fn main() {

    let listen_port = 8080;
    let debug_mode = true;

    let logger = Logger::new(debug_mode);
    let client_manager = ClientManager::new();
	let webserver = WebServer::new(listen_port, logger, client_manager);
	webserver.start();

}
