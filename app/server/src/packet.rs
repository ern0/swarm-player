#![allow(unused)]

use std::collections::HashMap;
use tinyjson::JsonValue;

type JsonObj = HashMap<String, JsonValue>;

pub struct Packet {
    packet_type: String,
    data_as_num: i64,
    data_as_str: String,
    stamp: i64,
}

impl Packet {
    
    pub fn new() -> Self {
        return Packet {
            packet_type: String::new(),
            data_as_num: 0, 
            data_as_str: String::new(),
            stamp: 0,
        };
    }

    pub fn get_type(&self) -> &String {
        return &self.packet_type;
    }

    pub fn get_num(&self) -> i64 {
        return self.data_as_num;
    }

    pub fn get_str(&self) -> &String {
        return &self.data_as_str;
    }

}

impl From<&String> for Packet {

    fn from(text: &String) -> Self {

        let parsed: JsonValue = text.parse().unwrap();
        let root_object: &JsonObj = parsed.get().unwrap();
        let (num_value, str_value) = parse_data(&root_object, 0, -1);

        return Packet {
            packet_type: parse_type(&root_object),
            data_as_num: num_value,
            data_as_str: str_value,
            stamp: 0,
        };
    }

}

fn parse_type(root_object: &JsonObj) -> String {

    let packet_type_value: &JsonValue = root_object.get("type").unwrap();
    if let JsonValue::String(string) = packet_type_value {
        return string.to_string();
    }
    
    return String::from("n.a.");
}

fn parse_data(root_object: &JsonObj, index: usize, undef: i64) -> (i64, String) {

    let data_vec: &JsonValue = root_object.get("data").unwrap();
    if let JsonValue::Array(vec) = data_vec {        
        let elem = &vec[index];

        match elem {
            JsonValue::Number(num) => {
                return (*num as i64, num.to_string(),);
            },
            JsonValue::String(str) => {
                return (undef, str.to_string(),);
            },
            _ => { },
        }
    }

    return (undef, undef.to_string(),);
}
