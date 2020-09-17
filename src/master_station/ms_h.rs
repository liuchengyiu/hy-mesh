#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
pub struct MeshMessage {
    pub timestamp: String,
    pub body: MeshMessageBody
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
pub struct MeshMessageBody {
    pub r#type: u8,
    pub len: u32,
    pub data: String
}

impl MeshMessage {
    pub fn new(data: String) -> MeshMessage {
        let r#type: u8 = 255;
        MeshMessage{
            token: token.get().to_string(), timestamp: dt, 
            body: MeshMessageBody{r#type: r#type, len: len, data: data}
        }
    }
}