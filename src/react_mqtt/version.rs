use std::collections::HashMap;
use std::sync::{Arc, Mutex};
extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct NodeVersion {
    pub version: HashMap<String, Vec<u8>>
}

lazy_static! {
    pub static ref NODEVERSION: Arc<Mutex<NodeVersion>>  = Arc::new(Mutex::new(NodeVersion::new()));
}

impl NodeVersion {
    pub fn new() -> NodeVersion {
        NodeVersion {
            version: HashMap::new()
        }
    }

    pub fn set_version(&mut self, mac: String, version: &[u8]) {
        self.version.insert(mac, version.to_vec().clone());
    }

    pub fn get_version(&self) -> String {
        let decoded: String = json::encode(&self).unwrap();
        decoded
    }
}