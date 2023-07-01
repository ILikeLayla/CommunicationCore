use super::data_class::{MessagesList, Message, User, DateTime, Local};
use super::replacer::Replace;
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};

pub struct Manager {
    channel: (Arc<Mutex<MessagesList>>, Arc<Mutex<MessagesList>>),
    comms: HashMap<String, MgHandler>,
}

pub struct MgHandler {
    buf: Arc<Mutex<MessagesList>>,
    all: Arc<Mutex<MessagesList>>,
    users: Arc<Mutex<Vec<User>>>,
    id: String,
    changed: Arc<Mutex<(bool, bool)>>,
}

impl PartialEq for MgHandler {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    } 
    
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl MgHandler {
    pub fn new(users: Vec<User>, id: String) -> Self {
        Self {
            buf: Arc::new(Mutex::new(MessagesList::new())),
            all: Arc::new(Mutex::new(MessagesList::new())),
            users: Arc::new(Mutex::new(users)),
            changed: Arc::new(Mutex::new((false, false))),
            id
        }
    }

    pub fn from(users: Vec<User>, id: String, messages: MessagesList) -> Result<Self, String> {
        let mut buf = MgHandler::new(users, id);
        if buf.messages_check(&messages) {
            for i in messages.iter() {
                buf.send(i.clone())
            };
            return Ok(buf)
        } else {
            Err("Unplanned user(s) include.".to_string())
        }

    }

    pub fn message_check(&self, message: &Message) -> bool {
        loop {
            if let Ok(users) = self.users.try_lock() {
                return users.contains(&message.from) && users.contains(&message.to)
            }
        }
    }

    pub fn messages_check(&self, messages: &MessagesList) -> bool {
        let mut users = HashSet::new();
        let mut out = true;
        for i in messages.iter() {
            users.insert(i.from.clone());
            users.insert(i.to.clone());
        };
        let planned_users = self.users.lock().unwrap();
        for i in users.iter() {
            out = out && planned_users.contains(i)
        };
        out
    }

    pub fn send(&mut self, message: Message) {
        let mut condition = self.changed.lock().unwrap();
        let mut list = self.buf.lock().unwrap();

        list.push(message);
        condition.0 = true;
    }
}