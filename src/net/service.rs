use super::{replacer::Replace, Channel, FullMessage};
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;

fn service(channel: &Channel, send: Arc<Mutex<HashMap<String, TcpStream>>>, recv: TcpListener) {
    let recv_channel = channel.clone();
    let send_channel = channel.clone();

    thread::spawn(move || {
        loop {
            for i in recv.incoming() {
                if let Ok(stream) = i {
                    let mut reader = BufReader::new(&stream);
                    let mut buf = Vec::new();
                    reader.read_until(b'\x03', &mut buf).unwrap();
                    let buf = FullMessage::from_rawdata(buf);
                    if let Ok(message) = buf {
                        recv_channel.send(*message);
                    }
                }
            }
        }
    });

    thread::spawn(move || {
        loop {
            let send_buf = send_channel.recv();
            for (id, stream) in send.try_lock().unwrap().iter_mut() {
                for message in send_buf.iter() {
                    if &message.mgr_id == id {
                        stream.write(message.to_rawdata().as_slice());
                        stream.write("\x03".as_bytes());
                    }
                }
            }
        }
    }); 
}

fn start_service(channel: &Channel, send)