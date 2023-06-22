use chrono::{DateTime, Local};
use std::rc::Rc;
use super::{User, Message};

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