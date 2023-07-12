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
            println!("---------[TYPE:{}]", message_type);

            if message_type == "req.clk" {
                let arg = self.parse_message_data_int(root_object, 0);
                println!("----------[DATA:{}]", arg);
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

    fn parse_message_data_int(&self, root_object: &HashMap<String, JsonValue>, index: usize) -> i32 {

        let data_vec: &JsonValue = root_object.get(&String::from("data")).unwrap();
        if let JsonValue::Array(vec) = data_vec {
            let elem = &vec[index];
            if let JsonValue::Number(num) = elem {
                return *num as i32;
            }
        }

        return 0;
    }

}

