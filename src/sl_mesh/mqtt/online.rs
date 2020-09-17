extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
use std::{
    sync::{Arc, Mutex},
};

lazy_static! {
    static ref ONLINE: Arc<Mutex<Online>> = Arc::new(Mutex::new(Online::new()));
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
struct Online {
    online: Vec<String>
}

impl Online {
    pub fn new() -> Online {
        Online {online: Vec::new()}
    }

    pub fn add(&mut self, mac: String) {
        for i in 0 .. self.online.len() {
            if self.online[i] == mac {
                return;
            }
        }
        self.online.push(mac.clone());
    }

    pub fn remove(&mut self, mac: String) {
        for i in 0 .. self.online.len() {
            if self.online[i] != mac {
                continue;
            }
            self.online.remove(i);
            break;
        }
    }
    pub fn get(&self) -> String {
        let d: String = json::encode(&self.online.clone()).unwrap();

        d
    }
}

pub fn get_online() -> String {
    ONLINE.lock().unwrap().get()
}

pub fn node_in(mac: &String) {
    ONLINE.lock().unwrap().add(mac.to_string());
}

pub fn node_out(mac: &String) {
    ONLINE.lock().unwrap().remove(mac.to_string());
}