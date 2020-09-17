extern crate paho_mqtt as mqtt;
extern crate chrono;
use chrono::prelude::*;
use crate::common::tool::trans_to_string;
use std::{
    sync::{Arc, Mutex},
};
lazy_static! {
    static ref TOKEN: Arc<Mutex<Token>> = Arc::new(Mutex::new(Token::new(0)));
}

struct Token {
    token: u16
}

impl Token {
    pub fn new(init_num: u16) -> Token {
        Token {token: init_num}
    }
    pub fn add(&mut self, add_num: u16) {
        self.token = self.token + add_num;
    }
    pub fn get(&self) -> u16 {
        self.token
    }
    pub fn set(&mut self, set_num: u16) {
        self.token = set_num;
    }
}

pub const TXTOPIC: &str = "rfmanage/notify/message/comlm/comlm";

pub struct MqttPaho {
    pub client: mqtt::AsyncClient
}

impl MqttPaho {
    pub fn publish(&mut self, topic: String, data: String) {
        let message = mqtt::Message::new_retained(topic, data, 1);
        self.client.publish(message);
    }
    pub fn set_client(&mut self, client: mqtt::AsyncClient) {
        self.client = client;
    }
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
pub struct MeshMessage {
    pub token: String,
    pub timestamp: String,
    pub body: MeshMessageBody
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
pub struct MeshMessageBody {
    pub r#type: u8,
    pub len: u32,
    pub data: String
}

impl MeshMessage {
    pub fn new(data: &[u8]) -> MeshMessage {
        let len: u32 = data.len() as u32 *2;
        let r#type: u8 = 255;
        let d_string: String = trans_to_string(data);
        let mut token = TOKEN.lock().unwrap();
        if  token.get() > 4096 {
            token.set(0);
        }
        token.add(1);
        let dt = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        MeshMessage{token: token.get().to_string(), timestamp: dt, 
                        body: MeshMessageBody{r#type: r#type, len: len, data: d_string}}
    }
}