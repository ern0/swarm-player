#![allow(unused)]

use std::collections::HashMap;
use std::vec::Vec;
use std::time::{SystemTime, UNIX_EPOCH};
use tinyjson::JsonValue;
use crate::utils::{UNDEF, STAMP_OFFSET_MS, systime_to_millis};

pub enum SyncMode { SyncData, AsyncCommand }

type JsonObj = HashMap<String, JsonValue>;

pub struct Packet {
    packet_type: String,
    data_as_num: Vec<i64>,
    data_as_str: Vec<String>,
}

impl Packet {

    pub fn new() -> Self {

        return Packet {
            packet_type: String::new(),
            data_as_num: Vec::new(),
            data_as_str: Vec::new(),
        };
    }

    pub fn new_simple_num(cmd: &str, parm: i64) -> Self {

        let mut packet = Packet::new();
        packet.set_type(&cmd);
        packet.set_num(0, parm);

        return packet;
    }

    #[allow(dead_code)]
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

    pub fn get_sync_mode(&self) -> SyncMode {
        if self.packet_type == "CLK_REF" {
            return SyncMode::AsyncCommand;
        } else {
            return SyncMode::SyncData;
        }
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

    #[allow(dead_code)]
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

        if num_value == UNDEF {
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

    pub fn get_bool(&self, index: usize) -> bool {

        let left_char = &self.data_as_str[index][0..1];
        let left_upcase = left_char.to_uppercase();

        return match left_upcase.as_str() {
            "F" => false,
            "N" => false,
            "0" => false,
            _ => true,
        };
    }

    pub fn render_json(&self, stamp: SystemTime) -> String {

        let stamp_millis = systime_to_millis(stamp);
        let mut json: String = String::from("{");
        
        json.push_key("type");
        json.push_quoted(self.get_type());
        
        if stamp != UNIX_EPOCH {
            json.push_str(",");
            json.push_key("stamp");
            match self.get_sync_mode() {
                SyncMode::SyncData => {
                    let offseted = stamp_millis + STAMP_OFFSET_MS;
                    json.push_str(&offseted.to_string());
                },
                SyncMode::AsyncCommand => {
                    json.push_str(&0.to_string());
                },
            }
        }
        
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
    use crate::packet::{Packet, SyncMode};
    use crate::utils::UNDEF;

    const JSON_BASIC_STR: &str = r#"{"type":"TYP","data":["VAL"]}"#;
    const JSON_BASIC_NUM: &str = r#"{"type":"BEAST","data":[666]}"#;
    const JSON_MULTI: &str = r#"{"type":"T","data":[10,11,12]}"#;
    const JSON_MIXED: &str = r#"{"type":"T","data":[0,"one",2]}"#;
    const JSON_ASYNC_CMD: &str = r#"{"type":"CLK_REF","data":[]}"#;
    const JSON_SYNC_DATA: &str = r#"{"type":"T","data":[]}"#;

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
        let json = packet.render_json(UNDEF);
        assert_eq!(json, JSON_BASIC_STR);
    }
    #[test]
    fn create_simple_num() {
        let mut packet = Packet::new_simple_num("BEAST", 666);
        let json = packet.render_json(UNDEF);
        assert_eq!(json, JSON_BASIC_NUM);
    }
    #[test]
    fn create_mixed() {
        let mut packet = Packet::new_simple_num("T", 0);
        packet.set_str(1, "one");
        packet.set_num(2, 2);
        let json = packet.render_json(UNDEF);
        assert_eq!(json, JSON_MIXED);
    }
    #[test]
    fn create_async_cmd() {
        let packet = Packet::from(&JSON_ASYNC_CMD.to_string());
        let sync_mode = packet.get_sync_mode();
        assert!(matches!(sync_mode, SyncMode::AsyncCommand));
    }
    #[test]
    fn create_sync_data() {
        let packet = Packet::from(&JSON_SYNC_DATA.to_string());
        let sync_mode = packet.get_sync_mode();
        assert!(matches!(sync_mode, SyncMode::SyncData));
    }

}
