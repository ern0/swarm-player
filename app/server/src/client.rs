//#![allow(unused)]

use std::collections::HashMap;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::thread::sleep;
use simple_websockets::{ Responder, Message };
use tinyjson::JsonValue;

pub struct Client {
	pub id: u64,
	pub responder: Responder,
    pub seen: i64,
    pub epoch: i64,
}

impl Client {

    pub fn new(id: u64, responder: Responder) -> Self {
        return Client {
            id: id,
            responder: responder,  
            seen: 0,         
            epoch: now(), 
        }
    }

	pub fn process_incoming_message(self: &mut Client, text: String) {		

        self.touch();

        let parsed: JsonValue = text.parse().unwrap();
        let root_object: &HashMap<String, JsonValue> = parsed.get().unwrap();
        let message_type = self.parse_message_type(root_object);

        if message_type == "CLK_0" {
            self.process_request_clk0(root_object);
        }

	}

    fn touch(&mut self) {
        self.seen = now();
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

    fn process_request_clk0(&self, root_object: &HashMap<String, JsonValue>) {

        let _clk0 = self.parse_message_data_int(root_object, 0);

        sleep(Duration::from_millis(1000));
        let clk_server = now();
        println!("REPORT: {}", clk_server - 1677710000000);
        sleep(Duration::from_millis(1000));

        ////println!("REPORT: {}", (clk_server - self.epoch));

        self.send_response_int(String::from("CLK_REF"), clk_server);
    }

    fn send_response_int(&self, command: String, value: i64) {
        
        let mut response: String = String::from("{");
        json_add_key(&mut response, "type");
        json_add_quoted(&mut response, &command);
        response.push_str(",");
        json_add_key(&mut response, "data");
        response.push_str("[");
        response.push_str(&value.to_string());
        response.push_str("]");
        response.push_str("}");

        self.send_response(response);
    }

    fn send_response(&self, response: String) {

        ////println!("send: [{}]", response);

        let message = Message::Text(response);
        self.responder.send(message);

    }
}


fn json_add_key(result: &mut String, value: &str) {

    json_add_quoted(result, value);
    result.push_str(":");
}

fn json_add_quoted(result: &mut String, value: &str ) {

    result.push_str("\"");
    result.push_str(value);
    result.push_str("\"");
}

fn now() -> i64 {

    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        as i64;
}