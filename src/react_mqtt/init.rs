extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
use crate::frame_deal::mesh_h::NodeStatus;
use crate::frame_lib::mesh::trans_to_string;
use super::online::*;
use super::topo::TOPO;
use super::nbr::NODENBR;
use super::net_test::NETTEST;
use super::white_list::WHITELIST;
use super::online;
use crate::mqtts::init;
use super::pan_id;
use super::node_leave;
use crate::mqtts::init::publish_message;
use crate::websocket::websocket_h::SocketMessage;

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