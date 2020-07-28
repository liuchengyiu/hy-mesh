extern crate websocket;
use std::thread;
use self::websocket::Message;
use self::websocket::sync::Server;
use self::websocket::OwnedMessage;
use super::websocket_h::*;
use crate::react_mqtt;
pub fn init() {
	let server = Server::bind("127.0.0.1:1234").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(move || {
			if !request.protocols().contains(&"rust-websocket".to_string()) {
                println!("reject");
                request.reject().unwrap();
				return;
			}

			let mut client = request.use_protocol("rust-websocket").accept().unwrap();

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			let message = OwnedMessage::Text("Hello".to_string());
			client.send_message(&message).unwrap();

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
                    OwnedMessage::Text(text) => {
                        println!("fuck you{}", &text);
                        let data = decode_string_to_socket_message(&text);
                        let mut response: String = String::from("");
                        match data {
                            Ok(result) => {
                                match result.event.as_str() {
                                    "hy-mesh/topo/get" => {
                                        response = react_mqtt::init::reponse_topo_get("hy-mesh/topo/response");
                                    },
                                    "hy-mesh/nbr/get" => {
                                        response = react_mqtt::init::reponse_nbr_get("hy-mesh/nbr/response");
                                    },
                                    "hy-mesh/whitelist/get" => {
                                        response = react_mqtt::init::reponse_whitelist_get("hy-mesh/whitelist/response");
                                    },
                                    "hy-mesh/online/get" => {
                                        response = react_mqtt::init::reponse_online_get("hy-mesh/online/response");
                                    },
                                    "hy-mesh/pan_id/set" => {
                                        react_mqtt::init::set_pan_id("rfmanage /notify/message/comlm/comlm", result.data.as_str());
                                    },
                                    "hy-mesh/command_node_leave/set" => {
                                        react_mqtt::init::command_node_leave("hy-mesh/command_node_leave/response", result.data.as_str());
                                    },
                                    _ => {
                                        println!("else");
                                    }
                                }
                            },
                            Err(_) => {
                                println!("error: socket message format is not right");
                            }

                        }
                        if response != String::from("") {
                            let message = OwnedMessage::Text(response);
                            sender.send_message(&message).unwrap();
                        }
                    }
                    _ => sender.send_message(&message).unwrap(),
				}
			}
		});
	}
}