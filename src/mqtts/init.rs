extern crate paho_mqtt as mqtt;
use super::subdeals;
use std::{
    env,
    process,
    thread,
    time::Duration,
};
use std::sync::{Arc, Mutex};
use super::mqtt_h::*;

const TOPICS: &[&str] = &["comlm/#", "hy-mesh/#", "rfmanage/#"];
const QOS: &[i32] = &[1,1,1]; 

lazy_static! {
//         pub static ref mqtt_paho_client: Arc<Mutex<MqttPaho<'static>>>= Arc::new(Mutex::new(MqttPaho{topics: HashMap::new()}));
    pub static ref MQTT_PAHO_CLIENT: Arc<Mutex<MqttPaho>> = Arc::new(Mutex::new(MqttPaho{client: mqtt::AsyncClientBuilder::new().finalize()}));
}

pub fn publish_message(topic: &str, data: String) {
    let mut client = MQTT_PAHO_CLIENT.lock().unwrap();

    println!("{}", topic);
    client.publish(topic.to_string(), data);
}

fn on_connect_success(cli: &mqtt::AsyncClient, _msgid: u16) {
    println!("Connection Succeeded");
    cli.subscribe_many(TOPICS, QOS);
    println!("sub scribing to topics: {:?}", TOPICS);
}

fn on_connect_failure(cli: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
    println!("Connection attempt failed with error code {}.\n", rc);
    thread::sleep(Duration::from_millis(2500));
    cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}

/////////////////////////////////////////////////////////////////////////////

pub fn init() {
    // Initialize the logger from the environment

    let host = env::args().nth(1).unwrap_or_else(||
        "tcp://127.0.0.1:1883".to_string()
    );

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("rust_async_subscribe")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    // Set a closure to be called whenever the client connection is established.
    cli.set_connected_callback(|_cli: &mqtt::AsyncClient| {
        println!("Connected.");
    });

    // Set a closure to be called whenever the client loses the connection.
    // It will attempt to reconnect, and set up function callbacks to keep
    // retrying until the connection is re-established.
    cli.set_connection_lost_callback(|cli: &mqtt::AsyncClient| {
        println!("Connection lost. Attempting reconnect.");
        thread::sleep(Duration::from_millis(2500));
        cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
    });

    // Attach a closure to the client to receive callback
    // on incoming messages.
    cli.set_message_callback(|_cli,msg| {
        if let Some(msg) = msg {
            let topic = msg.topic();
            let payload_str = msg.payload_str();
            println!("{} - {}", topic, payload_str);
            subdeals::res_data(&topic.to_string(), &payload_str.to_string());
        }
    });
    // subdeals::res_data(
    //     String::from("comlm/notify/message/rfmanage/rfmanage").trim().to_string(), 
    //     &String::from("{\"token\":\"2925\",\"timestamp\":\"2020-07-02T13:27:35Z\",\"body\":{\"type\":255,\"len\":30,\"data\":\"680f00a06963009001fcc23d26952800000000000000000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01fcc23d261f2c00000000000000000000ff9606630a000001fcc23d2629d7000000000000000000004b00a14843df16\"}}").trim().to_string());

    // Define the set of options for the connection
    let lwt = mqtt::Message::new("test", "Async subscriber lost connection", 1);

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(true)
        .will_message(lwt)
        .finalize();

    // Make the connection to the broker
    println!("Connecting to the MQTT server...");
    cli.connect_with_callbacks(conn_opts, on_connect_success, on_connect_failure);
    let mut client = MQTT_PAHO_CLIENT.lock().unwrap();
    client.set_client(cli);
    // Just wait for incoming messages.
    // Hitting ^C will exit the app and cause the broker to publish the
    // LWT message since we're not disconnecting cleanly.
}