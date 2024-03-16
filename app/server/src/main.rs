#![allow(unused)]

mod utils;
mod webserver;

use crate::webserver::WebServer;

pub fn main() {

	let webserver = WebServer::new(8080);
	webserver.start();

}
