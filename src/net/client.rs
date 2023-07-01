pub mod client {
    pub use std::net::{TcpListener, TcpStream};
    use std::io::{prelude::*, BufReader, Write};
    use std::str;
    use std::thread;
    use std::sync::{Arc, Mutex};
    use crate::net_protocol::replacer::Replace;
    use crate::data_class::{MessagesList, Message};

    fn client_service(stream:TcpStream) -> (Arc<Mutex<MessagesList>>, Arc<Mutex<MessagesList>>) {
        let send = Arc::new(Mutex::new(MessagesList::new()));
        let recv = Arc::new(Mutex::new(MessagesList::new()));
        let send_here = send.clone();
        let recv_here = recv.clone();
        
        let mut stream = stream;

        thread::spawn(move || {
            loop {
                if let Ok(mut send_buf) = send_here.lock() {
                    if let Ok(mut recv_buf) = recv_here.lock() {
                        for i in send_buf.iter() {
                            stream.write(i.to_rawdata().as_slice()).unwrap();
                            recv_buf.push(i.clone())
                        };
                        send_buf.clear();
                    }
                }
                stream.write("\x03".as_bytes()).unwrap();
                let mut reader = BufReader::new(&stream);
                let mut buf = Vec::new();
                reader.read_until(b'\x03', &mut buf).unwrap();
                recv_here.lock().unwrap().push(*Message::from_rawdata(buf).unwrap());
            }
        });

        (send, recv)
    }

    pub fn stater(addr: &str) -> (Arc<Mutex<MessagesList>>, Arc<Mutex<MessagesList>>) {
        let stream = TcpStream::connect(addr).unwrap();
        client_service(stream)
    }
}