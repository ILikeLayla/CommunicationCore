use super::data_class::{Message, User, DateTime, Local, FullMessagesList, FullMessage};
use super::replacer::Replace;
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};
use super::mg_handler::{MgHandler, Changed};
use super::channel::Channel;

pub struct Manager {
    channel: Channel,
    comms: HashMap<String, MgHandler>,
}

impl Manager {
    pub fn new(channel: Channel) -> Self {
        Self { 
            comms: HashMap::new(),
            channel
        }
    }

    pub fn update(&mut self) {
        let mut send_buf = Vec::new();
        for (id, mg) in &self.comms {
            if mg.changed.is_send() {
                let mut buf = mg.buf.lock().unwrap();
                for data in buf.iter() {
                    send_buf.push(data.clone())
                };
                buf.clear();
                
            }
        }

    }
}