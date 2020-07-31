use std::collections::HashMap;
use std::sync::{Arc, Mutex};
extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct NodeVersion {
    pub macs: Vec<Vec<u8>>,
    pub index: u8,
    pub version: HashMap<u8, Vec<u8>>
}

lazy_static! {
    pub static ref NODEVERSION: Arc<Mutex<NodeVersion>>  = Arc::new(Mutex::new(NodeVersion::new()));
}

impl NodeVersion {
    pub fn new() -> NodeVersion {
        NodeVersion {
            macs: Vec::new(),
            index: 0,
            version: HashMap::new()
        }
    }
    pub fn set(&mut self, macs: Vec<Vec<u8>>) {
        self.macs = macs.clone();
        self.index = 0;
        self.version = HashMap::new();
    }

    pub fn set_version(&mut self, version: &[u8]) {
        if self.index >= self.macs.len() as u8{
            return;
        }
        self.version.insert(self.index, version.to_vec().clone());
        self.index = self.index + 1;
    }

    pub fn get_next_mac(&mut self) -> Vec<u8> {
        match self.macs.get(self.index as usize) {
            Some(_) => {},
            None => {return vec![]}
        }
        self.macs[self.index as usize].clone()
    }

    pub fn get_version(&self) -> String {
        let decoded: String = json::encode(&self).unwrap();
        decoded
    }
}