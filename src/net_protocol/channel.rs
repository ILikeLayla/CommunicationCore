use super::data_class::FullMessagesList;
use std::sync::{Arc, Mutex};

pub struct Channel {
    recv: Arc<Mutex<FullMessagesList>>,
    send: Arc<Mutex<FullMessagesList>>
}

impl Channel {
    pub fn new(recv: Arc<Mutex<FullMessagesList>>, send: Arc<Mutex<FullMessagesList>>) -> Self {
        Self {
            recv, send
        }
    }

    pub fn update(&mut self, to_send: FullMessagesList) -> FullMessagesList {
        let mut send = self.send.lock().unwrap();
        for i in to_send.iter() {
            send.push(i.clone())
        };
        let recv = self.recv.lock().unwrap();
        recv.clone()
    }
}