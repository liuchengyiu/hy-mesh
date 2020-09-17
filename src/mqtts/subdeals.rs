extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
use crate::{sl_mesh, sl_mesh::lib::mesh::*, log};
use super::mqtt_h::*;
fn trans_to_vec(data: &String) -> Result<Vec<u8>, json::DecoderError> {
    let decoded: MeshMessage = json::decode(data)?;
    let frame_str = decoded.body.data;
    let mut frame_vec: Vec<u8> = Vec::new();
    let mut count: u8 = 0;
    let mut dec: u8 = 0;

    for b in frame_str.chars() {
        let after: u8 = hex_to_inter(b);
        if after == 16 {
            let temp: Vec<u8> = Vec::new();
            return Ok(temp);
        }
        match count {
            0 => {
                count = count + 1;
                dec = dec + after*16;
            },
            1 => {
                count = 0;
                dec = dec + after;
                frame_vec.push(dec);
                dec = 0;
            },
            _ => {
                break;
            }
        }
        continue;
    }
    Ok(frame_vec)
}
fn get_mesh_from_vec(frame: &[u8]) -> Vec<u8> {
    let mut mesh_frame: Vec<u8> = Vec::new();
    let mut index: usize = 0;
    
    while index < frame.len() {
        if frame[index] == 105 {
            if index+2 >= frame.len() {
                break;
            }
            let length: usize = (frame[index+1] as usize) + 256 * (frame[index+2] as usize);
            if index + length > frame.len() {
                index = index + 1;
                continue;
            }
            if frame[index + length - 1] != 67 {
                index = index + 1;
                continue;
            }
            for x in index..(index + length) {
                mesh_frame.push(frame[x]);
            }
            index = index + length;
            continue;
        }
        index = index + 1;
        continue;
    }
    mesh_frame
}

fn deal_by_type(frame: &[u8]) {
    match log::log_frame::log_mesh_frame(frame) {
        Ok(_) => println!("write ok"),
        Err(_) => println!("write err")
    }
    sl_mesh::processor::mesh::process_new_frame(frame);
}
fn deal_data(data: &String) {
    let mut frame: Vec<u8> = vec![];

    match trans_to_vec(data) {
        Ok(d) => {
            frame = d; 
        },
        Err(_) => {}
    }
    match frame.len() {
        0 =>{
            return;
        },
        _ => {},
    }
    let mesh_frame: Vec<u8> = get_mesh_from_vec(&frame);

    if frame_judge_crc16(&mesh_frame) == false {
        return;
    }
    deal_by_type(&mesh_frame);
}
pub fn res_data(topic: &String, data: &String) {
    let d: Result<MeshMessage, json::DecoderError> = json::decode(data);
    let mut result: MeshMessage = MeshMessage::new(&[0]);

    match d {
        Ok(d_) => {
            result = d_;
        },
        Err(_) => {
            return;
        }
    }

    match topic.trim().as_ref() {
        "comlm/notify/message/rfmanage/rfmanage" => {
            deal_data(data);
        },
        "hy-mesh/topo/get" => {
            sl_mesh::mqtt::init::reponse_topo_get("hy-mesh/topo/response");
        },
        "hy-mesh/nbr/get" => {
            sl_mesh::mqtt::init::reponse_nbr_get("hy-mesh/nbr/response");
        },
        "hy-mesh/whitelist/get" => {
            sl_mesh::mqtt::init::reponse_whitelist_get("hy-mesh/whitelist/response");
        },
        "hy-mesh/online/get" => {
            sl_mesh::mqtt::init::reponse_online_get("hy-mesh/online/response");
        },
        "hy-mesh/pan_id/set" => {
            sl_mesh::mqtt::init::set_pan_id("rfmanage/notify/message/comlm/comlm", result.body.data.as_str());
        },
        "hy-mesh/command_node_leave/set" => {
            sl_mesh::mqtt::init::command_node_leave("rfmanage/notify/message/comlm/comlm", result.body.data.as_str());
        },
        "hy-mesh/command_register/set" => {
            sl_mesh::mqtt::init::command_register("rfmanage/notify/message/comlm/comlm");
        },
        "hy-mesh/version/search" => {
            sl_mesh::mqtt::init::response_start_get_version("rfmanage/notify/message/comlm/comlm", result.body.data.as_str());
        },
        "hy-mesh/whitelist/set" => {
            sl_mesh::mqtt::init::response_whitelist_set(result.body.data.as_str());
        },
        "hy-mesh/version/get" => {
            sl_mesh::mqtt::init::response_version_get("hy-mesh/version/response");
        },
        "hy-mesh/net_test/set" => {
            sl_mesh::mqtt::init::response_net_test_set("hy-mesh/test/set/response",result.body.data.as_str());
        },
        "hy-mesh/net_test/start" => {
            sl_mesh::mqtt::init::response_net_test_start("hy-mesh/test/start/response");
        },
        "hy-mesh/net_test/stop" => {
            sl_mesh::mqtt::init::response_net_test_stop("hy-mesh/test/stop/response");
        },
        "hy-mesh/net_test/get" => {
            sl_mesh::mqtt::init::response_net_test_get("hy-mesh/net_test/response");
        },
        _ => {},
    }
}