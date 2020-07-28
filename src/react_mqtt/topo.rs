use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::string::String as String;
use crate::frame_lib::mesh::trans_to_string;
extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Topo {
    pub nodes: HashMap<String, Node>
}
#[derive(RustcDecodable, RustcEncodable)]
pub struct Node {
    pub node_mac: Vec<u8>,
    pub parent_mac: Vec<u8>
}

lazy_static! {
        pub static ref TOPO: Arc<Mutex<Topo>>  = Arc::new(Mutex::new(Topo{nodes: HashMap::new()}));
}

impl Topo {
    pub fn insert_route(&mut self, node: Vec<Vec<u8>>) {
        let mut node_mac: String = String::new(); 
        match node.get(1) {
            Some(_x) => {},
            None => return
        }
        if node[0].len() < 16 || node[1].len() < 16 {
            return;
        }
        node_mac = trans_to_string(&node[0]);
        self.nodes.insert(node_mac, Node{node_mac: node[0].clone(), parent_mac: node[1].clone()});
    } 
    pub fn rm_route(&mut self, node: Vec<Vec<u8>>) {
        let mut node_mac: String = String::new(); 
        match node.get(1) {
            Some(_x) => {},
            None => return
        }
        if node[0].len() < 16 || node[1].len() < 16 {
            return;
        }
        node_mac = trans_to_string(&node[0]);
        self.nodes.remove(&node_mac);
    }
    pub fn get_route(&self) -> String{
        let decoded: String = json::encode(&self.nodes).unwrap();
        decoded
    }
}