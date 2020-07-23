use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::string::String as String;
extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
#[derive(RustcDecodable, RustcEncodable)]
pub struct WhiteList {
    pub devices: HashMap<String, Device>
}
#[derive(RustcDecodable, RustcEncodable)]
pub struct Device {
    pub site: String,
    pub bar_code: Vec<u8>
}

lazy_static! {
    pub static ref WHITELIST: Arc<Mutex<WhiteList>>  = Arc::new(Mutex::new(WhiteList{devices: HashMap::new()}));
}

impl WhiteList {
    pub fn insert_device(&mut self, mac: String, device: Device ) {
        self.devices.insert(mac, Device{bar_code: device.bar_code.clone(), site: device.site});
    } 
    pub fn rm_route(&mut self, mac: String) {
        self.devices.remove(&mac);
    }
    pub fn get_list(&self) -> String{
        let decoded: String = json::encode(&self.devices).unwrap();
        decoded
    }
}