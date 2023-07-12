#![allow(unused)]

use std::collections::HashMap;
use std::vec::Vec;
use tinyjson::JsonValue;
use crate::utils::UNDEF;

type JsonObj = HashMap<String, JsonValue>;

pub struct Packet {
    packet_type: String,
    data_as_num: Vec<i64>,
    data_as_str: Vec<String>,
    stamp: i64,
}

impl Packet {
    pub fn new() -> Self {

        return Packet {
            packet_type: String::new(),
            data_as_num: Vec::new(),
            data_as_str: Vec::new(),
            stamp: 0,
        };
    }

    pub fn get_type(&self) -> &String {
        return &self.packet_type;
    }

    pub fn get_num(&self, index: usize) -> i64 {        
        let num = self.data_as_num[index];
        if (num == UNDEF) {
            panic!("attempt to read string as number");
        }
        return num;
    }

    pub fn get_str(&self, index: usize) -> &String {
        return &self.data_as_str[index];
    }
}

impl From<&String> for Packet {

    fn from(text: &String) -> Self {

        let parsed: JsonValue = text.parse().unwrap();
        let root_object: &JsonObj = parsed.get().unwrap();
        let mut num_vec: Vec<i64> = Vec::new();
        let mut str_vec: Vec<String> = Vec::new();

        let packet_type = parse_type(&root_object);
        parse_data(&root_object, &mut num_vec, &mut str_vec);

        return Packet {
            packet_type: packet_type,
            data_as_num: num_vec,
            data_as_str: str_vec,
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

fn parse_data(root_object: &JsonObj, num_vec: &mut Vec<i64>, str_vec: &mut Vec<String>) {

    let data_arr: &JsonValue = root_object.get("data").unwrap();
    if let JsonValue::Array(vec) = data_arr {

        for index in 0..vec.len() {
            let elem = &vec[index];

            match elem {
                JsonValue::Number(num) => {
                    num_vec.push(*num as i64);
                    str_vec.push(num.to_string());
                }
                JsonValue::String(str) => {
                    num_vec.push(UNDEF);
                    str_vec.push(str.to_string());
                }
                _ => {}
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::packet::Packet;

    const JSON_BASIC: &str = r#"{"type":"TYP","data":["VAL"]}"#;
    const JSON_MULTI: &str = r#"{"type":"T","data":[10,11,12]}"#;
    const JSON_MIXED: &str = r#"{"type":"T","data":[0,"one",2]}"#;

    #[test]
    fn parse_simple_type() {
        let packet = Packet::from(&JSON_BASIC.to_string());
        assert_eq!(packet.get_type(), "TYP");
    }
    #[test]
    fn parse_simple_str() {
        let packet = Packet::from(&JSON_BASIC.to_string());
        assert_eq!(packet.get_str(0), "VAL");
    }
    #[test]
    fn parse_multi_num() {
        let packet = Packet::from(&JSON_MULTI.to_string());
        assert_eq!(packet.get_num(0), 10);
        assert_eq!(packet.get_num(1), 11);
        assert_eq!(packet.get_num(2), 12);
    }
    #[test]
    fn parse_mixed() {
        let packet = Packet::from(&JSON_MIXED.to_string());
        assert_eq!(packet.get_num(0), 0);
        assert_eq!(packet.get_str(1), "one");
        assert_eq!(packet.get_num(2), 2);
    }
    #[test]
    #[should_panic]
    fn parse_index_overrun() {
        let packet = Packet::from(&JSON_MIXED.to_string());
        _ = packet.get_num(4);
    }
    #[test]
    fn parse_num_as_str() {
        let packet = Packet::from(&JSON_MIXED.to_string());
        assert_eq!(packet.get_str(0), "0");
    }
    #[test]
    #[should_panic]
    fn parse_str_as_num() {
        let packet = Packet::from(&JSON_MIXED.to_string());
        _ = packet.get_num(1);
    }

}
