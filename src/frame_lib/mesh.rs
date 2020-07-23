pub fn create_69_frame(head: u8, r#type: u8, data: &[u8], end: u8) -> Vec<u8> {
    let length: u16 = 7 + data.len() as u16;
    let mut frame: Vec<u8> = Vec::new();
    frame.push(head);
    if length < 256 {
        frame.push(length as u8);
        frame.push(0);
    } else {
        frame.push((length % 256) as u8);
        frame.push((length / 256) as u8);
    }
    frame.push(r#type);
    for x in data {
        frame.push(*x)
    }
    let crc: u16 = crc16_a001(&frame);
    if crc < 256 {
        frame.push(crc as u8);
        frame.push(0)
    } else {
        frame.push((crc % 256) as u8);
        frame.push((crc / 256) as u8);
    }
    frame.push(end);
    frame
}

pub fn create_68_frame(head: u8, r#type: u8, data: &[u8], end: u8) -> Vec<u8> {
    let length: u16 = 6 + data.len() as u16;
    let mut frame: Vec<u8> = Vec::new();
    frame.push(head);
    if length < 256 {
        frame.push(length as u8);
        frame.push(0);
    } else {
        frame.push((length % 256) as u8);
        frame.push((length / 256) as u8);
    }
    frame.push(r#type);
    for x in data {
        frame.push(*x)
    }
    let mut sum: u8 = 0;
    for i in &frame {
        sum = i + sum;
    }
    frame.push(sum);
    frame.push(end);
    frame
}

fn crc16_a001(data: &[u8]) -> u16 {
    let mut crc16: u16 = 0xFFFF;
    
    for i in data {
        crc16 = crc16 ^ (*i as u16) ;
        for _j in 0..8 {
            if (crc16 & 0x01) > 0 {
                crc16 = (crc16 >> 1) ^ 0xa001;
                continue;
            }
            crc16 = crc16 >> 1;
            continue;
        }
    }
    crc16
}

pub fn frame_judge_crc16(data: &Vec<u8>) -> bool {
    let (left, right) = data.split_at(data.len() - 3 );
    let crc_cal: u16 = crc16_a001(&left);
    let crc_now : u16 = (right[0] as u16) + (right[1] as u16) * 256;
    if crc_now == crc_cal {
        return true;
    }
    false
}

pub fn hex_to_inter(hex: char) -> u8 {
    match hex {
        // '0' => 0,
        // '1' => 1,
        // '2' => 2,
        // '3' => 3,
        // '4' => 4,
        // '5' => 5,
        // '6' => 6,
        // '7' => 7,
        // '8' => 8,
        // '9' => 9,
        e @ '0' ..= '9' => (e as u8) - 48 ,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => 16
    }
}

pub fn inter_to_hex(hex: u8) -> char {
    match hex {
        e @ 0 ..= 9 => (e + 48) as char,
        e @ 10 ..= 15 => (e + 87) as char,
        _e @ 16 ..= 255 => 'x'
    }
}

pub fn trans_to_vec(data: &String) -> Vec<u8> {
    let mut frame_vec: Vec<u8> = Vec::new();
    let mut count: u8 = 0;
    let mut dec: u8 = 0;

    for b in data.chars() {
        let after: u8 = hex_to_inter(b);
        if after == 16 {
            let temp: Vec<u8> = Vec::new();
            return temp;
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
    frame_vec
}

pub fn trans_to_string(data: &[u8]) -> String {
    let mut d_string: String = String::new();
    for i in data {
        d_string.push(inter_to_hex((i / 16) as u8));
        d_string.push(inter_to_hex((i % 16) as u8));
    }
    d_string
}