use super::net;
use super::net_protocol;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

pub fn start(send: Arc<Mutex<HashMap<String, TcpStream>>>, recv: TcpListener) -> net_protocol::Manager {
    let channel = net_protocol::Channel::new();
    
    let manager = net_protocol::Manager::new(channel.clone());
    net::start_service(channel, send, recv);

    manager
}
