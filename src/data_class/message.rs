use chrono::{DateTime, Local};
use std::rc::Rc;
use super::User;

pub struct Message {
    pub time: DateTime<Local>,
    pub from: Rc<User>,
    pub to: Rc<User>,
    pub text: String,
}

impl Message {
    pub fn new(from: Rc<User>, to: Rc<User>, text: String) -> Self {
        Self {
            time: Local::now(),
            from, text, to
        }
    }
}

impl std::fmt::Display for Message{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message< time:{}, text:{}, from:{}, to:{} >", self.time, self.text, self.from, self.to)
    }
}

