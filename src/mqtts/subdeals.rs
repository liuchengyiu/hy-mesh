extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
use crate::frame_lib::mesh::*;
use crate::log;
use crate::frame_deal;
use super::mqtt_h::*;
use crate::react_mqtt::init;
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
                println!("bad loop");
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
    // match log::log_frame::log_mesh_frame(frame) {
    //     Ok(_) => println!("write ok"),
    //     Err(_) => println!("write err")
    // }
    frame_deal::mesh::process_new_frame(frame);
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
            println!("Sorry, this vector is too short");
            return;
        },
        _ => println!("continue!"),
    }
    let mesh_frame: Vec<u8> = get_mesh_from_vec(&frame);

    if frame_judge_crc16(&mesh_frame) == false {
        return;
    }
    deal_by_type(&mesh_frame);
}
pub fn res_data(topic: &String, data: &String) {
    match topic.trim().as_ref() {
        "comlm/notify/message/rfmanage/rfmanage" => {
            deal_data(data);
        },
        _ => println!("mqtt: something else!"),
    }
}