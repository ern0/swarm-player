#![allow(unused)]

mod webserver;

use crate::webserver::WebServer;

pub fn main() {

	let webserver = WebServer::new();
	webserver.start();

}
