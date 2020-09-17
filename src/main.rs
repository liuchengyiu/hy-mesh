use std::{thread, time::Duration};
use mesh_monitor::{sl_mesh, mqtts};

fn main() {
    sl_mesh::processor::mesh::init_mesh_processors();
    let handle = thread::spawn(move || {
        mqtts::init::init();
        loop {
            thread::sleep(Duration::from_millis(1000));
        }
    });
    handle.join().unwrap();
}