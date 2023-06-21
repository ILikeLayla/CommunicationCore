pub mod client {
    pub use std::net::{TcpListener, TcpStream};
    use std::io::{prelude::*, BufReader, Write};
    use std::str;
    use std::thread;
    pub use std::sync::mpsc::{channel, Sender, Receiver};

    fn client_service(stream:TcpStream) -> (Sender<Vec<u8>>, Receiver<Vec<u8>>) {
        let (sender_local, receiver_here) = channel::<Vec<u8>>();
        let (sender_here, reciever_local) = channel();
        let mut stream = stream;

        thread::spawn(move || {
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
        });

        (sender_local, reciever_local)
    }

    pub fn stater(addr: &str) -> (Sender<Vec<u8>>, Receiver<Vec<u8>>) {
        let stream = TcpStream::connect(addr).unwrap();
        client_service(stream)
    }
}