use chrono::{DateTime, Local};
use std::rc::Rc;

pub struct User {
    pub name: String
}

impl User {
    pub fn new(name:String) -> Self {
        Self {
            name
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User< name:{} >", self.name)
    }
}

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



pub struct MessagesList {
    pub messages: Vec<Message>,
}

impl MessagesList {
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            messages
        }
    }

    pub fn push(&mut self, message:Message) {
        self.messages.insert(0, message)
    }

    pub fn changeFromUser(&mut self, user: Rc<User>) {
        for i in self.messages.iter_mut() {
            i.from = user.clone();
        }
    }

    pub fn changeToUser(&mut self, user: Rc<User>) {
        for i in self.messages.iter_mut() {
            i.to = user.clone()
        }
    }
}