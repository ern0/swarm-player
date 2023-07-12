#![allow(unused)]

use std::collections::HashMap;
use simple_websockets::{ Responder, Message };
use tinyjson::JsonValue;

pub struct Client {
	pub id: u64,
	pub responder: Responder,
}

impl Client {

	pub fn process_incoming_message(self: &Client, message: Message) {		
    
        if let Message::Text(text) = message {

            let parsed: JsonValue = text.parse().unwrap();
            let root_object: &HashMap<_, _> = parsed.get().unwrap();
            let message_type = self.parse_message_type(root_object);
            println!("---------[{}]", message_type);

            if message_type == "req.clk" {
                //let arg = self.parse_message_data_int(root_object, 1);
                //process_request_clock(arg);
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

}

