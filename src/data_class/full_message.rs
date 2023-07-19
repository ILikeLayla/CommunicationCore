use super::{Message, Replace, Outputer};

#[derive(Clone)]
pub struct FullMessage {
    message: Message,
    mgr_id: String
}

impl FullMessage {
    pub fn new(id: String, message:Message) -> Self {
        Self { message, mgr_id: id }
    }

    pub fn get_data(&self) -> Message {
        self.message.clone()
    }

    pub fn get_id(&self) -> String {
        self.mgr_id.clone()
    }
}

impl Replace for FullMessage {
    fn from_rawdata(rawdata: Vec<u8>) -> Result<Box<Self>, String> {
        rawdata.output("Message", |data| {
            let mut buf = Vec::new();
            let data = &data[12..(data.len()-1)];
            let data = std::str::from_utf8(&data.to_vec()).unwrap().to_string();
            for i in data.split(",") {
                buf.push(i.to_string())
            };

            Self::new(buf[0].clone(), *Message::from_rawdata(buf[1].as_bytes().to_vec()).unwrap())
        }, "#Dfr05")
    }

    fn to_rawdata(&self) -> Vec<u8> {
        format!("Message<{:?},{:?}>", self.mgr_id.to_rawdata(), self.message.to_rawdata()).as_bytes().to_vec()
    }
}
