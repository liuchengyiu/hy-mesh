extern crate chrono;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::SeekFrom;
use std::io::Write;
use std::io::Read;
use std::io::prelude::*;
use self::chrono::prelude::*;
fn get_file_object() -> Result<File, std::io::Error> {
    let log_file_path: String = "log".to_string();
    let path = Path::new(&log_file_path);
    match path.exists() {
        true => {
            let mut f = OpenOptions::new().write(true).read(true).open(path)?;
            let len = f.metadata()?.len();
            println!("Metadata length :{:?}", f.metadata());
            if len < 4 {
                let pointer:Vec<u8> = vec![4, 0, 0, 0];
                match f.write(&pointer) {
                    Ok(_) => {
                       return Ok(f);
                    },
                    Err(err) => {
                        println!("Error:{}", err);
                        return Err(err);
                    } 
                }    
            }
            return Ok(f);
        },
        false => {
            let mut f = OpenOptions::new().write(true).read(true).create(true).open(path)?;
            let pointer:Vec<u8> = vec![4, 0, 0, 0];
            match f.write(&pointer) {
                Ok(_) => {
                   return Ok(f);
                },
                Err(err) => {
                   return Err(err);
                } 
            }
        }
    } 
}
fn get_file_pointer() -> Result<u32, std::io::Error> {
    let mut file = get_file_object()?;
    let mut file_point = [0; 4];
    let mut now_file_pointer:u32 = 0;
    file.seek(SeekFrom::Start(0))?;
    file.read(&mut file_point)?;
    println!("{:?}", file_point);
    for j in 0..4 {
        now_file_pointer = now_file_pointer + file_point[j] as u32 * 256u32.pow(j as u32);
    }
    if now_file_pointer > 2u32.pow(25) {
        now_file_pointer = 4
    }
    Ok(now_file_pointer)
}

fn gen_log_frame(frame: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let local: DateTime<Local> = Local::now();
    let mut log_frame: Vec<u8> = vec![];
    log_frame.push((local.year() % 256) as u8 );
    log_frame.push((local.year() / 256) as u8 );
    log_frame.push(local.month() as u8 );
    log_frame.push(local.day() as u8 );
    log_frame.push(local.hour() as u8 );
    log_frame.push(local.second() as u8 );
    for i in frame {
        log_frame.push(*i)
    }
    println!("{:?}", log_frame);
    Ok(log_frame)
}

fn write_pointer(file:&mut File, pointer: u32) -> Result<(), std::io::Error> {
    let mut pointer_vec:Vec<u8> = vec![];
    pointer_vec.push( pointer as u8);
    pointer_vec.push((pointer >> 8) as u8);
    pointer_vec.push((pointer >> 16) as u8);
    pointer_vec.push((pointer >> 24) as u8);
    file.seek(SeekFrom::Start(0))?;
    file.write(&pointer_vec)?;
    Ok(())
}

pub fn log_mesh_frame(frame: &[u8]) -> Result<(), std::io::Error> {
    let now_file_pointer = get_file_pointer()?;
    let log_frame: Vec<u8> = gen_log_frame(&frame)?;
    let mut file = get_file_object()?;
    
    let after_pointer = now_file_pointer + log_frame.len() as u32;
    file.seek(SeekFrom::Start(now_file_pointer as u64))?;
    file.write(&log_frame)?;
    write_pointer(&mut file, after_pointer)?;

    Ok(())
}
