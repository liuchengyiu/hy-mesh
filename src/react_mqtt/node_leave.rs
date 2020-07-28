extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
use crate::frame_lib::mesh::*;
use crate::mqtts::mqtt_h::MeshMessage;

#[derive(RustcDecodable, RustcEncodable)]
pub struct NodeLeave {
    pub mac: String,
    pub duration: u32
}

impl NodeLeave {
    pub fn new(node_leave: &str) -> NodeLeave {
        let decoded: NodeLeave = json::decode(&node_leave).unwrap();
        decoded
    }

    pub fn get_frame_json(&self) -> String {
        let mut data: Vec<u8> = vec![];
        let mac: Vec<u8> = trans_to_vec(&self.mac);

        data.push(1);
        for i in &mac {
            data.push(*i);
        }

        if mac.len() != 16 {
            return "".to_string();
        }

        data.push((self.duration & 0x000000FF) as u8);
        data.push(((self.duration & 0x0000FF00) >> 8) as u8);
        data.push(((self.duration & 0x00FF0000) >> 16) as u8);
        data.push(((self.duration & 0xFF000000) >> 24) as u8);
        let frame_69 = create_69_frame(105, 17, &data, 67);
        let frame_68 = create_68_frame(104, 32, &frame_69, 22);

        let message: MeshMessage = MeshMessage::new(&frame_68);

        json::encode(&message).unwrap()
    }
}
