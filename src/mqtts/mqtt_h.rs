extern crate paho_mqtt as mqtt;
use crate::frame_lib::mesh::trans_to_string;
static mut TOKEN: u16 = 0;
pub const TXTOPIC: &str = "rfmanage/notify/message/comlm/comlm";
// pub struct MqttPaho<'a> {
//     pub topics: HashMap<String, mqtt::topic::Topic<'a>>
// }

// impl<'a> MqttPaho<'a> {
//     pub fn publish(&mut self, topic: String, data: String) {
//         let message = mqtt::Message::new(topic, data, 1);
        
//     }
// }
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
pub struct MeshMessage {
    pub token: String,
    pub timestamp: String,
    pub body: MeshMessageBody
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MeshMessageBody {
    pub r#type: u8,
    pub len: u32,
    pub data: String
}

impl MeshMessage {
    pub fn new(data: &[u8]) -> MeshMessage {
        let mut token: u16 = 0;
        let len: u32 = data.len() as u32 *2;
        let r#type: u8 = 255;
        let d_string: String = trans_to_string(data);
        unsafe {
            if TOKEN > 4096 {
                TOKEN = 0;
            }
            token = TOKEN;
            TOKEN = TOKEN + 1;
        }
        MeshMessage{token:token.to_string(), timestamp:"".to_string(), 
                        body: MeshMessageBody{r#type: r#type, len: len, data: d_string}}
    }
}