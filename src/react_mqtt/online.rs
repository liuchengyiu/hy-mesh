extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;

static mut ONLINE: Vec<String> = vec![];

pub fn get_online() -> String {
    unsafe {
        json::encode(&ONLINE).unwrap()
    }
}

pub fn node_in(mac: &String) {
    unsafe {
        for i in 0 .. ONLINE.len() {
            if &ONLINE[i] == mac {
                return;
            }
        }
        ONLINE.push(mac.clone());
    }
}

pub fn node_out(mac: &String) {
    unsafe {
        for i in 0 .. ONLINE.len() {
            if &ONLINE[i] != mac {
                continue;
            }
            ONLINE.remove(i);
            break;
        }
    }
}