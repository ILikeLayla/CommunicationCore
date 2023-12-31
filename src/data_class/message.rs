use chrono::{DateTime, Local};
use std::rc::Rc;
use super::{Outputer, User};

#[derive(Clone, PartialEq, Eq)]
pub struct Message {
    time: DateTime<Local>,
    from: Rc<User>,
    to: Rc<User>,
    text: String,
}

impl Message {
    pub fn new(from: Rc<User>, to: Rc<User>, text: &str) -> Self {
        Self {
            time: Local::now(),
            text: text.to_string(),
            from, to
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_to(&self) -> Rc<User> {
        self.to.clone()
    }

    pub fn get_from(&self) -> Rc<User> {
        self.from.clone()
    }

    pub fn get_time(&self) -> DateTime<Local> {
        self.time.clone()
    }
}

impl std::fmt::Display for Message{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message< time:{}, text:{}, from:{}, to:{} >", self.time, self.text, self.from, self.to)
    }
}

impl super::Replace for Message {
    fn from_rawdata(rawdata: Vec<u8>) -> Result<Box<Self>, String> {
        rawdata.output("Message", |data| {
            let mut buf = Vec::new();
            let data = &data[8..(data.len()-1)];
            let data = std::str::from_utf8(&data.to_vec()).unwrap().to_string();
            for i in data.split("\x1F") {
                buf.push(i.to_string())
            };

            Message::new(
                Rc::new(*User::from_rawdata(buf[1].as_bytes().to_vec()).unwrap()), 
                Rc::new(*User::from_rawdata(buf[2].as_bytes().to_vec()).unwrap()), 
                &buf[0]
            )
        }, "#Dfr04")
    }

    fn to_rawdata(&self) -> Vec<u8> {
        format!("Message<{}\x1F{}\x1F{}>", self.text, self.from, self.to).as_bytes().to_vec()
    }
}