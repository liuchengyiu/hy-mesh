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