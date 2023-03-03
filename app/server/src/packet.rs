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

    pub fn new_simple_num(cmd: &str, parm: i64) -> Self {

        let mut packet = Packet::new();
        packet.set_type(&cmd);
        packet.set_num(0, parm);

        return packet;
    }

    pub fn new_simple_str(cmd: &str, parm: &str) -> Self {

        let mut packet = Packet::new();
        packet.set_type(&cmd);
        packet.set_str(0, &parm);

        return packet;
    }

    pub fn set_type(&mut self, packet_type: &str) {
        self.packet_type = String::from(packet_type);
    }

    pub fn get_type(&self) -> &String {
        return &self.packet_type;
    }

    pub fn set_num(&mut self, index: usize, value: i64) {
        
        if self.data_as_num.len() == index {
            self.data_as_num.push(value);
            self.data_as_str.push(value.to_string());
        } else {
            self.data_as_num[index] = value;
            self.data_as_str[index] = value.to_string();
        }

    }

    pub fn set_str(&mut self, index: usize, value: &str) {

        if self.data_as_num.len() == index {
            self.data_as_num.push(UNDEF);
            self.data_as_str.push(value.to_string());
        } else {
            self.data_as_num[index] = UNDEF;
            self.data_as_str[index] = value.to_string();
        }

    }

    pub fn get_num(&self, index: usize) -> i64 {        

        let num_value = self.data_as_num[index];

        if (num_value == UNDEF) {
            panic!(
                "attempt to read string as number, type=\"{}\" value=\"{}\"", 
                &self.get_type(),                
                &self.data_as_str[index]
            );
        }

        return num_value;
    }

    pub fn get_str(&self, index: usize) -> &String {
        return &self.data_as_str[index];
    }

    pub fn render_json(&self) -> String {

        let mut json: String = String::from("{");
        json.push_key("type");
        json.push_quoted(self.get_type());
        json.push_str(",");
        json.push_key("data");
        json.push_str("[");

        let mut first = true;
        for index in 0 .. self.data_as_num.len() {

            if first {
                first = false;
            } else {
                json.push_str(",")
            }

            let num_value = &self.data_as_num[index];
            let str_value = &self.data_as_str[index];
            if *num_value == UNDEF {
                json.push_quoted(str_value);
            } else {
                json.push_str(str_value);
            }          

        }

        json.push_str("]");
        json.push_str("}");

        return json;

    }

}

trait JsonCreatorExt {
    fn push_num(&mut self, value: i64);
    fn push_quoted(&mut self, value: &str);
    fn push_key(&mut self, key: &str);
}

impl JsonCreatorExt for String {

    fn push_num(&mut self, value: i64) {
        self.push_str(&value.to_string());
    }

    fn push_quoted(&mut self, value: &str) {
        self.push_str("\"");
        self.push_str(value);
        self.push_str("\"");
    }

    fn push_key(&mut self, key: &str) {
        self.push_quoted(key);
        self.push_str(":");
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

        for index in 0 .. vec.len() {
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

    const JSON_BASIC_STR: &str = r#"{"type":"TYP","data":["VAL"]}"#;
    const JSON_BASIC_NUM: &str = r#"{"type":"BEAST","data":[666]}"#;
    const JSON_MULTI: &str = r#"{"type":"T","data":[10,11,12]}"#;
    const JSON_MIXED: &str = r#"{"type":"T","data":[0,"one",2]}"#;

    #[test]
    fn parse_simple_type() {
        let packet = Packet::from(&JSON_BASIC_STR.to_string());
        assert_eq!(packet.get_type(), "TYP");
    }
    #[test]
    fn parse_simple_str() {
        let packet = Packet::from(&JSON_BASIC_STR.to_string());
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

    #[test]
    fn create_simple_str() {
        let mut packet = Packet::new_simple_str("TYP","VAL");
        let json = packet.render_json();
        assert_eq!(json, JSON_BASIC_STR);
    }
    #[test]
    fn create_simple_num() {
        let mut packet = Packet::new_simple_num("BEAST", 666);
        let json = packet.render_json();
        assert_eq!(json, JSON_BASIC_NUM);
    }
    #[test]
    fn create_mixed() {
        let mut packet = Packet::new_simple_num("T", 0);
        packet.set_str(1, "one");
        packet.set_num(2, 2);
        let json = packet.render_json();
        assert_eq!(json, JSON_MIXED);
    }

}
