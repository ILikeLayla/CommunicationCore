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
        let mut send_buf = FullMessagesList::new();
        for (id, mg) in self.comms.iter() {
            if mg.changed.is_send() {
                let mut buf = mg.buf.lock().unwrap();
                for data in buf.iter() {
                    send_buf.push(FullMessage::new(id.clone(), data.clone()))
                };
                buf.clear(); 
            }
        }
        for (_, mg) in self.comms.iter_mut() {
            mg.changed.handled_send()
        }
        
        let recv = self.channel.update(send_buf);

        for fmg in recv.iter() {
            for (id, mg) in self.comms.iter_mut() {
                if id == &fmg.mgr_id {
                    mg.push(fmg.message.clone());
                    mg.changed.recv();
                }
            }
        }
    }

    pub fn join(&mut self, mg: MgHandler) {
        self.comms.insert(mg.id.clone(), mg);
    }
}