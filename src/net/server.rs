pub mod server {
    pub use std::net::{TcpListener, TcpStream};
    use std::io::{self, prelude::*, BufReader, Write, Error, Read};
    use std::thread;
    use std::str;
    pub use std::sync::mpsc::{channel, Sender, Receiver};

    pub fn server_service(listener: TcpListener) -> (Sender<Vec<u8>>, Receiver<Vec<u8>>) {
        let (sender_local, receiver_here) = channel::<Vec<u8>>();
        let (sender_here, reciever_local) = channel();

        println!("start");

        thread::spawn( move || {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    let mut stream = stream;
                    loop {
                        if let Ok(data) = receiver_here.try_recv() {
                            stream.write(data.as_slice()).unwrap();
                        }
                        stream.write("\x03".as_bytes()).unwrap();
                        let mut reader = BufReader::new(&stream);
                        let mut buf = Vec::new();
                        reader.read_until(b'\x03', &mut buf).unwrap();
                        sender_here.send(buf).unwrap();
                    }
                } else {
                    continue;
                }
                // break;
            }
        });
        
        (sender_local, reciever_local)
        
    }

    pub fn stater(addr: &str) -> (Sender<Vec<u8>>, Receiver<Vec<u8>>) {
        let listener = TcpListener::bind(addr).unwrap();
        server_service(listener)
    }
}