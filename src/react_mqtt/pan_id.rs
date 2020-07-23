extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
use frame_lib::mesh::*;
use mqtts::mqtt_h::MeshMessage;

#[derive(RustcDecodable, RustcEncodable)]
pub struct PanID {
    pub mac: String,
    pub pan_id: String
}

impl PanID {
    pub fn new(pan: &str) -> PanID {
        let decoded: PanID = json::decode(&pan).unwrap();
        decoded
    }

    pub fn get_frame_json(&self) -> String {
        let mut data: Vec<u8> = vec![];
        let mac: Vec<u8> = trans_to_vec(&self.mac);
        let pan_id: Vec<u8> = trans_to_vec(&self.pan_id);

        data.push(1);
        for i in &mac {
            data.push(*i);
        }

        if mac.len() != 16 || pan_id.len() != 2 {
            return "".to_string();
        }

        data.push(pan_id[0]);
        data.push(pan_id[1]);
        let frame_69 = create_69_frame(105, 27, &data, 67);
        let frame_68 = create_68_frame(104, 32, &frame_69, 22);

        let message: MeshMessage = MeshMessage::new(&frame_68);

        json::encode(&message).unwrap()
    }
}

