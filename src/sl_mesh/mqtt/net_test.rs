use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    string::String as String,
    sync::mpsc::*,
    thread,
    time::Duration
};
use crate::mqtts::{
    mqtt_h::TXTOPIC,
    init::publish_message,
    mqtt_h::MeshMessage
};
use crate::sl_mesh::lib::mesh::*;
extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
pub struct NetTest {
    pub test: Test,
    pub tx: Arc<Mutex<Sender<String>>>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Test {
    pub flag: bool,
    pub res: HashMap<String, u16>,
    pub send: HashMap<String, u16>,
    pub interval: u16,
    pub duration: u16,
}

lazy_static! {
    pub static ref NETTEST: Arc<Mutex<NetTest>>  = Arc::new(Mutex::new(NetTest::new()));
}

impl NetTest {
    pub fn init(&mut self, interval: u16, duration: u16) {
        use super::white_list::WHITELIST;
        let white_list = WHITELIST.lock().unwrap();
        let keys = white_list.devices.keys().clone();
        self.test.res = HashMap::new();
        self.test.send = HashMap::new();
        self.test.flag = false;
        for key in keys {
            self.test.res.insert(key.clone(), 0);
            self.test.send.insert(key.clone(), 0);
        }
        self.test.interval = interval;
        self.test.duration = duration;
    }

    pub fn start_test(&mut self) {
        if self.test.flag == true {
            return;
        }
        self.test.flag = true;
        self.tx.lock().unwrap().send(json::encode(&self.test).unwrap());
    }

    pub fn recode_rx(&mut self, mac: String) {
        if self.test.flag == false {
            return;
        }
        let mut time: u16 = 0;
        match self.test.res.get(&mac) {
            Some(d) => {
                time = *d;
            },
            None => return
        }
        self.test.res.insert(mac, time + 1);
    }

    pub fn recode_tx(&mut self, mac: String) {
        if self.test.flag == false {
            return;
        }
        let mut time: u16 = 0;
        match self.test.send.get(&mac) {
            Some(d) => {
                time = *d + 1;
            },
            None => return
        }
        self.test.send.insert(mac, time);
    }

    pub fn stop_test(&mut self) {
        if self.test.flag == false {
            return;
        }
        self.test.flag = false;
        self.tx.lock().unwrap().send("stop".to_string()).unwrap();
    }

    pub fn new() -> NetTest{
        let (sender, receiver) = channel();
        let sender = Arc::new(Mutex::new(sender));
        let receiver = Arc::new(Mutex::new(receiver));

        new_thread(Arc::clone(&receiver));
        NetTest {test: Test {flag: false, res: HashMap::new(), 
                            send: HashMap::new(), interval: 0, duration: 0}, 
                tx: sender}
    }
    pub fn get_test(&self) -> String{
        let decoded: String = json::encode(&self.test).unwrap();
        
        decoded
    }
}

fn new_thread(receiver: Arc<Mutex<Receiver<String>>>) {
    thread::spawn(move || loop{
        let rx = receiver.lock().unwrap();
        let message = rx.recv().unwrap();
        if message ==  "error".to_string() || message == "stop".to_string() {
            continue;
        }
        let test: Test = json::decode(&message).unwrap();

        if test.flag == false || test.interval < 4 || test.duration < test.send.len() as u16 * test.interval {
            continue;
        }
        let mut nodes: Vec<String> = vec![];
        for key in test.send.keys() {
            nodes.push(key.clone());
        }

        let mut time: u16 = 0;
        let mut index: usize = 0;
        'outer:while time < test.duration {
            if index >= nodes.len() {
                index = 0;
            }
            match nodes.get(index) {
                Some(_) => {},
                None => break 'outer
            }
            let mac: String = nodes[index].clone();
            let data: Vec<u8> = trans_to_vec(&mac);
            let frame_69 = create_69_frame(105, 23, &data, 67);
            let frame_68 = create_68_frame(104, 32, &frame_69, 22);
            let message: MeshMessage = MeshMessage::new(&frame_68);
            let mut sleep_time = 0;
            publish_message(TXTOPIC, json::encode(&message).unwrap());
            {
                NETTEST.lock().unwrap().recode_tx(mac);
            }
            while sleep_time < test.interval {
                thread::sleep(Duration::from_secs(1));
                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Terminating.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
                sleep_time = sleep_time + 1;
            }
            time = time + test.interval;
            index = index + 1;
        }
        {
            NETTEST.lock().unwrap().test.flag = false;
        }
    });
}