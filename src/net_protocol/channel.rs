use super::data_class::{FullMessage, FullMessagesList};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Channel {
    recv: Arc<Mutex<FullMessagesList>>,
    send: Arc<Mutex<FullMessagesList>>
}

impl Channel {
    pub fn new() -> Self {
        Self {
            recv: Arc::new(Mutex::new(FullMessagesList::new())),
            send: Arc::new(Mutex::new(FullMessagesList::new())),
        }
    }

    pub fn from(recv: Arc<Mutex<FullMessagesList>>, send: Arc<Mutex<FullMessagesList>>) -> Self {
        Self {
            recv, send
        }
    }

    pub fn update(&self, to_send: FullMessagesList) -> FullMessagesList {
        let mut send = self.send.lock().unwrap();
        for i in to_send.iter() {
            send.push(i.clone())
        };
        let recv = self.recv.lock().unwrap();
        recv.clone()
    }

    pub fn send_list(&self, to_send: FullMessagesList) {
        let mut send = self.send.lock().unwrap();
        for i in to_send.iter() {
            send.push(i.clone())
        };
    }

    pub fn send(&self, to_send: FullMessage) {
        let mut send = self.send.lock().unwrap();
        send.push(to_send.clone())
    }

    pub fn recv(&self) -> FullMessagesList {
        let recv = self.recv.lock().unwrap();
        recv.clone()
    }
}
