use super::data_class::{MessagesList, Message, User, DateTime, Local};
use super::replacer::Replace;
use std::sync::{Arc, Mutex};

pub struct Manager {
    buf: MessagesList,
    all: Arc<Mutex<MessagesList>>,
    channel: Arc<Mutex<MessagesList>>,
    users: Vec<User>,
}

impl Manager {
    pub fn new(users: Vec<User>, channel: (Arc<Mutex<MessagesList>>, Arc<Mutex<MessagesList>>)) -> Self {
        Self {
            buf: MessagesList::new(),
            all: channel.1,
            channel: channel.0,
            users
        }
    }

    pub fn from(users: Vec<User>, channel: (Arc<Mutex<MessagesList>>, Arc<Mutex<MessagesList>>), send_buf: MessagesList) -> Result<Self, String> {
        let mut buf = Self::new(users, channel);
        if let Err(a) = buf.user_check_list(&send_buf) {
            Err(a)
        } else {
            let _ = buf.send_list_to_buf(send_buf);
            Ok(buf)
        }
    }

    pub fn user_check_one(&self, message: &Message) -> Result<(), String> {
        if (!self.users.contains(&*message.from)) || (!self.users.contains(&*message.to)) {
            Err("Unplanned user".to_string())
        } else {
            Ok(())
        }
    }

    pub fn user_check_list(&self, messages: &MessagesList) -> Result<(), String> {
        for i in messages.messages.iter() {
            if let Err(_) = self.user_check_one(i) {
                return Err("Unplanned user(s) include".to_string())
            }
        };
        Ok(())
    }

    pub fn send_list_to_buf(&mut self, messages: MessagesList) -> Result<(), String> {
        if let Err(a) = self.user_check_list(&messages) {
            Err(a)
        } else {
            for i in messages.iter() {
                self.buf.push(i.clone())
            };
            Ok(())
        }
    }

    pub fn send_to_buf(&mut self, message:Message) -> Result<(), String>{
        if let Err(a) = self.user_check_one(&message) {
            Err(a)
        } else {
            self.buf.push(message);
            Ok(())
        }
    }

    pub fn send_all(&self) {
        loop {
            if let Ok(mut buf) = self.channel.lock() {
                for i in self.buf.send_out().iter() {
                    buf.push(i.clone())
                };
                buf.clear();
                break;
            };
        }
    }
}