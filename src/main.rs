extern crate hy_mesh;
use std::thread;
use hy_mesh::frame_deal;
use std::time::Duration;
// use hy_mesh::websocket::init;
fn main() {
    frame_deal::mesh::init_mesh_processors();
    let handle = thread::spawn(move || {
        hy_mesh::mqtts::init::init();
        loop {
            thread::sleep(Duration::from_millis(1000));
        }
    });
    // let websocket = thread::spawn(move || {
    //     init::init();
    //     loop {
    //         thread::sleep(Duration::from_millis(1000));
    //     }
    // });
    handle.join().unwrap();
}