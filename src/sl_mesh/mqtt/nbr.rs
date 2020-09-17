use crate::sl_mesh::{processor::mesh_h::NetNbr, lib::mesh::trans_to_string};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    string::String as String
};
extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
#[derive(RustcDecodable, RustcEncodable)]
pub struct NodeNbr {
    pub nbrs: HashMap<String, Vec<NetNbr>>
}

lazy_static! {
    pub static ref NODENBR: Arc<Mutex<NodeNbr>>  = Arc::new(Mutex::new(NodeNbr{nbrs: HashMap::new()}));
}

impl NodeNbr {
    pub fn insert_nbr(&mut self, mac: &[u8] ,nbr: &Vec<NetNbr>) {
        let mut node_mac: String = String::new(); 
        if mac.len() < 16 {
            return;
        }
        node_mac = trans_to_string(&mac);
        self.nbrs.insert(node_mac, nbr.clone());
    }
    pub fn rm_route(&mut self, mac: &[u8]) {
        let mut node_mac: String = String::new(); 
        if mac.len() < 16  {
            return;
        }
        node_mac = trans_to_string(&mac);
        self.nbrs.remove(&node_mac);
    }
    pub fn get_route(&self) -> String{
        let decoded: String = json::encode(&self.nbrs).unwrap();
        decoded
    }
}
