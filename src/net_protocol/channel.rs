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

    pub fn update(&mut self, send: FullMessagesList) -> FullMessagesList {
        let mut buf = self.send.lock().unwrap();
        for i in send.iter() {
            buf.push(i.clone())
        };
        let mut buf = self.recv.lock().unwrap();
        let out = buf.clone();
        buf.clear();
        out
    }
}