// pub mod server {
//     use std::net::TcpListener;
//     use std::io::{prelude::*, BufReader, Write};
//     use std::thread;
//     use std::str;
//     use std::sync::{Arc, Mutex};
//     use crate::net_protocol::{replacer::Replace, channel::Channel};
//     use crate::data_class::{FullMessagesList, FullMessage};

//     pub fn server_service(listener: TcpListener) -> Channel {
//         let send = Arc::new(Mutex::new(FullMessagesList::new()));
//         let recv = Arc::new(Mutex::new(FullMessagesList::new()));
//         let send_here = send.clone();
//         let recv_here = recv.clone();

//         println!("start");

//         thread::spawn( move || {
//             // for stream in listener.incoming() {
//             //     // rewrite the net handle logic. 
//             //     if let Ok(stream) = stream {
//             //         let mut stream = stream;
//             //         loop {
//             //             if let Ok(mut send_buf) = send_here.lock() {
//             //                 if let Ok(mut recv_buf) = recv_here.lock() {
//             //                     for i in send_buf.iter() {
//             //                         stream.write(i.to_rawdata().as_slice()).unwrap();
//             //                         recv_buf.push(i.clone())
//             //                     };
//             //                     send_buf.clear();
//             //                 }
//             //             }

//             //             stream.write("\x03".as_bytes()).unwrap();
//             //             let mut reader = BufReader::new(&stream);
//             //             let mut buf = Vec::new();
//             //             reader.read_until(b'\x03', &mut buf).unwrap();
//             //             recv_here.lock().unwrap().push(*FullMessage::from_rawdata(buf).unwrap());
//             //         }
//             //     } else {
//             //         continue;
//             //     }
//             //     // break;
//             // }
//         });
        
//         // Channel::new(recv, send)
        
//     }

//     pub fn stater(addr: &str) -> Channel {
//         let listener = TcpListener::bind(addr).unwrap();
//         server_service(listener)
//     }
// }