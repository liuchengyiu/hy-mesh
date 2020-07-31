extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
use crate::frame_deal::mesh_h::NodeStatus;
use crate::frame_lib::mesh::trans_to_string;
use super::online::*;
use super::topo::TOPO;
use super::nbr::NODENBR;
use super::net_test::NETTEST;
use super::white_list::WHITELIST;
use super::version::NODEVERSION;
use super::online;
use crate::mqtts::init;
use super::pan_id;
use super::node_leave;
use crate::mqtts::init::publish_message;
use crate::websocket::websocket_h::SocketMessage;
use crate::mqtts::mqtt_h::MeshMessage;
use crate::frame_lib::mesh::create_69_frame;
use crate::frame_lib::mesh::create_68_frame;

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
struct Device {
    pub mac: String,
    pub site: String,
    pub bar_code: String,
}

pub fn deal_node_status(node_status: NodeStatus) {
    let mac: String = trans_to_string(&node_status.mac);

    node_in(&mac);
    if node_status.net_routes.len() > 0 {
        let mut route: Vec<Vec<u8>> = vec![];

        route.push(node_status.mac.clone());
        route.push(node_status.net_routes[0].parent_mac.clone());
        TOPO.lock().unwrap().insert_route(route);
    }

    if node_status.nbrs.len() > 0 {
        NODENBR.lock().unwrap().insert_nbr(&node_status.mac, &node_status.nbrs);
    }
}

pub fn deal_node_leave(node_mac: Vec<Vec<u8>>) {
    for i in node_mac {
        let mac: String = trans_to_string(&i);

        node_out(&mac);
    }
}

pub fn recv_node_reponse(mac_array: &[u8]) {
    let mac: String = trans_to_string(mac_array);
    
    NETTEST.lock().unwrap().recode_rx(mac);
}

pub fn recv_node_version(version: &[u8]) {
    let mut next_mac: Vec<u8> = Vec::new();
    
    {
        let mut node_version = NODEVERSION.lock().unwrap();
        node_version.set_version(version);
        next_mac = node_version.get_next_mac();
    }
    if next_mac.len() != 16 {
        return;
    }

    let mut data: Vec<u8> = Vec::new();

    for i in &next_mac {
        data.push(*i);
    }

    let frame_69 = create_69_frame(105, 42, &data, 67);
    let frame_68 = create_68_frame(104, 32, &frame_69, 22);
    let message: MeshMessage = MeshMessage::new(&frame_68);
    let s = json::encode(&message).unwrap();
    
    publish_message("rfmanage/notify/message/comlm/comlm", s);
}

pub fn reponse_topo_get(topic: &str) -> String {
    let topo_str: String = TOPO.lock().unwrap().get_route();
    json::encode(&SocketMessage{
        event: topic.to_string(),
        data: topo_str
    }).unwrap()
    // init::publish_message(topic, topo_str);
}

pub fn reponse_nbr_get(topic: &str) -> String {
    let nbr_str: String =  NODENBR.lock().unwrap().get_route();
    json::encode(&SocketMessage{
        event: topic.to_string(),
        data: nbr_str
    }).unwrap()
    // init::publish_message(topic, nbr_str);
}

pub fn reponse_whitelist_get(topic: &str) -> String {
    let whitellist_str: String = WHITELIST.lock().unwrap().get_list();

    json::encode(&SocketMessage{
        event: topic.to_string(),
        data: whitellist_str
    }).unwrap()
    // init::publish_message(topic, whitellist_str);
}

pub fn reponse_online_get(topic: &str) -> String {
    let online_str: String = online::get_online();
    json::encode(&SocketMessage{
        event: topic.to_string(),
        data: online_str
    }).unwrap()
    // init::publish_message(topic, online_str);
}

pub fn set_pan_id(topic: &str, data: &str) {
    let pan_id: pan_id::PanID = pan_id::PanID::new(data);
    let frame_string: String = pan_id.get_frame_json();

    if frame_string == "".to_string() {
        return;
    }
    publish_message(topic, frame_string);
}

pub fn command_node_leave(topic: &str, data: &str) {
    let node_leave: node_leave::NodeLeave = node_leave::NodeLeave::new(data);
    let frame_string: String = node_leave.get_frame_json();

    if frame_string == "".to_string() {
        return;
    }

    publish_message(topic, frame_string);
}

pub fn command_register(topic: &str) {
    let mut data: Vec<u8> = Vec::new();
    data.push(255);
    let frame_69 = create_69_frame(105, 19, &data, 67);
    let frame_68 = create_68_frame(104, 32, &frame_69, 22);

    let message: MeshMessage = MeshMessage::new(&frame_68);

    let s = json::encode(&message).unwrap();
    
    publish_message(topic, s);
}

pub fn start_get_version(topic: &str, data: &str) {
    let macs: Result<Vec<Vec<u8>>, json::DecoderError> = json::decode(data);

    match macs {
        Ok(mac) => {
            {
                let mut node_version = NODEVERSION.lock().unwrap();
                node_version.set(mac.clone()); 
            }
            if mac.len() == 0 {
                return;
            }
            let first = &mac[0];
            if first.len()  != 16 {
                return;
            }
            let mut data: Vec<u8> = Vec::new();

            for i in first {
                data.push(*i);
            }
        
            let frame_69 = create_69_frame(105, 42, &data, 67);
            let frame_68 = create_68_frame(104, 32, &frame_69, 22);
            let message: MeshMessage = MeshMessage::new(&frame_68);
            let s = json::encode(&message).unwrap();
            
            publish_message(topic, s);
        },
        Err(_) => {}
    }
}

pub fn version_get(topic: &str) -> String {
    let online_str: String = NODEVERSION.lock().unwrap().get_version();

    json::encode(&SocketMessage{
        event: topic.to_string(),
        data: online_str
    }).unwrap()
}

pub fn whitelist_set(data: &str) {
    let result: Result<Vec<Device>, json::DecoderError> = json::decode(data);

    match result {
        Ok(lists) => {
            if lists.len() == 0 {
                println!("whist list data LEN == 0");
                return;
            }
            {
                let mut white_list = WHITELIST.lock().unwrap();

                white_list.clear();
                for device in lists {
                    white_list.insert_device(device.mac, device.site, device.bar_code);
                }
            }
        },
        Err(_) => {
            println!("whist list data error");
            return;
        }
    }
}