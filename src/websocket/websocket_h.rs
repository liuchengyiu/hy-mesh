extern crate rustc_serialize as rustc_serialize;
use self::rustc_serialize::json;
#[derive(RustcDecodable, RustcEncodable)]
pub struct SocketMessage {
    pub event: String,
    pub data: String,
}

pub fn decode_string_to_socket_message(data: &String) -> Result<SocketMessage, rustc_serialize::json::DecoderError> {
    let message: SocketMessage = json::decode(&data)?;
    Ok(message)
}