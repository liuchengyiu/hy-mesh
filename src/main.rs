extern crate hy_mesh;
use std::thread;
use hy_mesh::frame_deal;
fn main() {
    frame_deal::mesh::init_mesh_processors();
    let handle = thread::spawn(move || {
        hy_mesh::mqtts::init::init();
        "END"
    });
    println!("{}", handle.join().unwrap());
}