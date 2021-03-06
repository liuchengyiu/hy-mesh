extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
use super::{
    online::*,
    topo::TOPO,
    nbr::NODENBR,
    net_test::NETTEST,    
    white_list::WHITELIST,
    version::NODEVERSION,
    online,
    pan_id,
    node_leave,
};
use crate::mqtts::{
    init,
    init::publish_message 
};
use crate::mqtts::mqtt_h::MeshMessage;
use crate::sl_mesh::{
    processor::mesh_h::NodeStatus,
    lib::mesh
};
use std::{time::Duration, thread};
// use crate::websocket::websocket_h::SocketMessage;

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
struct Device {
    pub mac: String,
    pub site: String,
    pub bar_code: String,
}

pub fn deal_node_status(node_status: NodeStatus) {
    let mac: String = mesh::trans_to_string(&node_status.mac);

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
        let mac: String = mesh::trans_to_string(&i);

        node_out(&mac);
    }
}

pub fn recv_node_reponse(mac_array: &[u8]) {
    let mac: String = mesh::trans_to_string(mac_array);
    
    NETTEST.lock().unwrap().recode_rx(mac);
}

pub fn recv_node_version(mac: String, version: &[u8]) {

    
    {
        let mut node_version = NODEVERSION.lock().unwrap();
        node_version.set_version(mac, version);
    }
}

pub fn reponse_topo_get(topic: &str) {
    let topo_str: String = TOPO.lock().unwrap().get_route();

    init::publish_message(topic, topo_str);
}

pub fn reponse_nbr_get(topic: &str) {
    let nbr_str: String =  NODENBR.lock().unwrap().get_route();

    init::publish_message(topic, nbr_str);
}

pub fn reponse_whitelist_get(topic: &str) {
    let whitellist_str: String = WHITELIST.lock().unwrap().get_list();

    init::publish_message(topic, whitellist_str);
}

pub fn reponse_online_get(topic: &str) {
    let online_str: String = online::get_online();

    init::publish_message(topic, online_str);
}

pub fn set_pan_id(topic: &str, data: &str) {
    let pan_id: pan_id::PanID = pan_id::PanID::new(data);
    let frame_string: String = pan_id.get_frame_json();

    if frame_string == "".to_string() {
        return;
    }
    publish_message(topic, frame_string);
    publish_message("hy-mesh/pan_id/response", "{}".to_string());
}

pub fn command_node_leave(topic: &str, data: &str) {
    let node_leave: node_leave::NodeLeave = node_leave::NodeLeave::new(data);
    let frame_string: String = node_leave.get_frame_json();

    if frame_string == "".to_string() {
        return;
    }

    publish_message(topic, frame_string);
    publish_message("hy-mesh/command_node_leave/response", "{}".to_string());
}

pub fn command_register(topic: &str) {
    let mut data: Vec<u8> = Vec::new();
    data.push(255);
    let frame_69 = mesh::create_69_frame(105, 19, &data, 67);
    let frame_68 = mesh::create_68_frame(104, 32, &frame_69, 22);

    let message: MeshMessage = MeshMessage::new(&frame_68);

    let s = json::encode(&message).unwrap();
    
    publish_message(topic, s);
}

pub fn response_start_get_version(topic: &str, data: &str) {
    let macs: Result<Vec<Vec<u8>>, json::DecoderError> = json::decode(data);

    match macs {
        Ok(mac) => {
            if mac.len() == 0 {
                return;
            }
            for i in 0 .. mac.len() {
                let first = &mac[i];
                if first.len()  != 16 {
                    return;
                }
                let mut data: Vec<u8> = Vec::new();

                for i in first {
                    data.push(*i);
                }
        
                let frame_69 = mesh::create_69_frame(105, 42, &data, 67);
                let frame_68 = mesh::create_68_frame(104, 32, &frame_69, 22);
                let message: MeshMessage = MeshMessage::new(&frame_68);
                let s = json::encode(&message).unwrap();
            
                publish_message(topic, s);
                thread::sleep(Duration::from_millis(50));
            }
        },
        Err(_) => {}
    }
}

pub fn response_version_get(topic: &str) {
    let node_version_str: String = NODEVERSION.lock().unwrap().get_version();

    publish_message(topic, node_version_str);
}

pub fn response_whitelist_set(data: &str) {
    let result: Result<Vec<Device>, json::DecoderError> = json::decode(data);

    match result {
        Ok(lists) => {
            if lists.len() == 0 {
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
            return;
        }
    }
}

pub fn response_net_test_set(topic: &str, data: &str) {
    let result: Result<Vec<u16>, json::DecoderError> = json::decode(data); 

    match result {
        Ok(config) => {
            match config.get(1) {
                Some(_) => {},
                None => {
                    publish_message(topic, "test config data error".to_string());
                    return;
                }
            }
            {
                let mut net_test = NETTEST.lock().unwrap();
                if net_test.test.flag == true {
                    publish_message(topic, "test is already started".to_string());
                    return;
                }
                net_test.init(config[0], config[1]);
            }
        },
        Err(_) => {
            publish_message(topic, "test config data error".to_string());
            return;
        }
    }
    publish_message(topic, "test config success".to_string());
}

pub fn response_net_test_stop(topic: &str) {
    {
        let mut net_test = NETTEST.lock().unwrap();
        if net_test.test.flag == false {
            publish_message(topic, "test is already stopped".to_string());
            return;
        }
        net_test.stop_test();
    }
    publish_message(topic, "test success".to_string());
}

pub fn response_net_test_start(topic: &str) {
    {
        let mut net_test = NETTEST.lock().unwrap();
        if net_test.test.flag == true {
            publish_message(topic, "test is already started".to_string());
            return;
        }
        net_test.start_test();
    }
    publish_message(topic, "test success".to_string());  
}

pub fn response_net_test_get(topic: &str) {
    let net_test_str: String = NETTEST.lock().unwrap().get_test();

    publish_message(topic, net_test_str);
}