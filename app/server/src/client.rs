#![allow(unused)]

use std::collections::HashMap;
use std::sync::{ Arc };
use std::time::{SystemTime, UNIX_EPOCH};
use simple_websockets::{ Responder, Message };
use tinyjson::JsonValue;
use crate::client_manager::ClientManager;

pub struct Client {
	pub id: u64,
	pub responder: Responder,
}

impl Client {

    pub fn new(id: u64, responder: Responder) -> Self {
        return Client {
            id: id,
            responder: responder,            
        }
    }

	pub fn process_incoming_message(self: &Client, message: Message) {		
    
        if let Message::Text(text) = message {

            let parsed: JsonValue = text.parse().unwrap();
            let root_object: &HashMap<_, _> = parsed.get().unwrap();
            let message_type = self.parse_message_type(root_object);

            println!("---------[TYPE:{}]", message_type);

            if message_type == "CLK0" {
                let clk0 = self.parse_message_data_int(root_object, 0);
                let clk_server = self.process_request_clk0(clk0);
                self.send_response_int(String::from("CLKR"), clk_server);
            }
        }

	}

    fn parse_message_type(&self, root_object: &HashMap<String, JsonValue>) -> String {

        let type_value: &JsonValue = root_object.get(&String::from("type")).unwrap();
        if let JsonValue::String(string) = type_value {
            return string.to_string();
        }

        return String::from("n.a.");
    }

    fn parse_message_data_int(&self, root_object: &HashMap<String, JsonValue>, index: usize) -> i64 {

        let data_vec: &JsonValue = root_object.get(&String::from("data")).unwrap();
        if let JsonValue::Array(vec) = data_vec {
            let elem = &vec[index];
            if let JsonValue::Number(num) = elem {
                return *num as i64;
            }
        }

        return 0;
    }

    fn process_request_clk0(&self, clk0: i64) -> i64 {

        let clk_server = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            as i64;

        return clk_server;
    }

    fn send_response_int(&self, command: String, value: i64) {
        
        let mut response: String = String::from("{");
        add_key(&mut response, "type");
        add_quoted(&mut response, "CLKR");
        response.push_str(",");
        add_key(&mut response, "data");
        response.push_str("[");
        response.push_str(&value.to_string());
        response.push_str("]");
        response.push_str("}");

        self.send_response(response);
    }

    fn send_response(&self, response: String) {

        println!("send: [{}]", response);

        let message = Message::Text(response);
        self.responder.send(message);

    }
}


fn add_key(result: &mut String, value: &str) {

    add_quoted(result, value);
    result.push_str(":");
}

fn add_quoted(result: &mut String, value: &str ) {

    result.push_str("\"");
    result.push_str(value);
    result.push_str("\"");
}