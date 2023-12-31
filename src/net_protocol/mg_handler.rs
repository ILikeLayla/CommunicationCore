use std::sync::{Arc, Mutex};
use super::data_class::{MessagesList, Message, User};

pub struct MgHandler {
    pub buf: Arc<Mutex<MessagesList>>,
    all: MessagesList,
    // users: Arc<Mutex<Vec<User>>>,
    pub id: String,
    pub changed: Arc<Mutex<Condition>>,
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
    pub fn new(id: String) -> Self {
        Self {
            buf: Arc::new(Mutex::new(MessagesList::new())),
            all: MessagesList::new(),
            changed: Arc::new(Mutex::new(Condition::new())),
            id
        }

        // with user
        //     Self {
        //         buf: Arc::new(Mutex::new(MessagesList::new())),
        //         all: MessagesList::new(),
        //         users: Arc::new(Mutex::new(users)),
        //         changed: Arc::new(Mutex::new(Condition::new())),
        //         id
        //     }
    }

    pub fn from(id: String, messages: MessagesList) -> Result<Self, String> {
        let mut buf = MgHandler::new(id);
        for i in messages.iter() {
            buf.send(i.clone())
        };
        return Ok(buf)

        // with user
        //     let mut buf = MgHandler::new(users, id);
        //     if buf.messages_check(&messages) {
        //         for i in messages.iter() {
        //             buf.send(i.clone())
        //         };
        //         return Ok(buf)
        //     } else {
        //         Err("Unplanned user(s) include.".to_string())
        //     }
    }

    // =====================================================================
    // user check

    // pub fn message_check(&self, _message: &Message) -> bool {
    //     if let Ok(users) = self.users.lock() {
    //         return users.contains(&message.from) && users.contains(&message.to)
    //     } else {
    //         false
    //     }
    // }

    // pub fn messages_check(&self, _messages: &MessagesList) -> bool {
    //     let mut users = HashSet::new();
    //     let mut out = true;
    //     for i in messages.iter() {
    //         users.insert(i.from.clone());
    //         users.insert(i.to.clone());
    //     };
    //     let planned_users = self.users.lock().unwrap();
    //     for i in users.iter() {
    //         out = out && planned_users.contains(i)
    //     };
    //     out
    // }
    // =====================================================================

    pub fn send(&mut self, message: Message) {
        let mut condition = self.changed.lock().unwrap();
        let mut list = self.buf.lock().unwrap();

        self.all.push(message.clone());
        list.push(message);
        condition.send();
    }

    pub fn push(&mut self, message: Message) {
        self.all.push(message);
        self.changed.recv();
    }
}

pub struct Condition {
    send: bool,
    recv: bool,
}

impl Condition {
    fn new() -> Self {
        Self { 
            send: false, recv: false
        }
    }
}

pub trait Changed {
    fn send(&mut self);
    fn recv(&mut self);
    fn reset(&mut self);
    fn is_changed(&self) -> bool;
    fn is_send(&self) -> bool;
    fn is_recv(&self) -> bool;
    fn handled_send(&mut self);
}

impl Changed for Condition {
    fn recv(&mut self) {
        self.recv = true
    }

    fn send(&mut self) {
        self.send = true
    }

    fn reset(&mut self) {
        (self.recv, self.send) = (false, false)
    }

    fn is_send(&self) -> bool {
        self.send
    }

    fn is_recv(&self) -> bool {
        self.recv
    }

    fn is_changed(&self) -> bool {
        self.recv || self.send
    }

    fn handled_send(&mut self) {
        self.send = false
    }
}

impl Changed for Arc<Mutex<Condition>> {
    fn recv(&mut self) {
        self.lock().unwrap().recv()
    }

    fn send(&mut self) {
        self.lock().unwrap().send()
    }

    fn reset(&mut self) {
        self.lock().unwrap().reset()
    }

    fn is_changed(&self) -> bool {
        self.lock().unwrap().is_changed()
    }

    fn is_recv(&self) -> bool {
        self.lock().unwrap().is_recv()
    }

    fn is_send(&self) -> bool {
        self.lock().unwrap().is_send()
    }

    fn handled_send(&mut self) {
        self.lock().unwrap().handled_send()
    }
}