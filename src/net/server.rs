pub mod Server {
    pub use std::net::{TcpListener, TcpStream};
    use std::io::{self, prelude::*, BufReader, Write, Error, Read};
    use std::str;
    pub use std::sync::mpsc;

    pub fn server_service(listener: &TcpListener) {
        // let (senderLocal, receiverHere) = channel::<T>();
        // let mut stream = stream;
        // let (senderHere, recieverLocal) = channel();
        // let listener = TcpListener::bind(addr).unwrap();
        // let mut buf = Vec::new();

        println!("start");

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            // println!("{:?}", buf);
            // let recieve = stream.read(&mut buf).unwrap();
            // println!("{}", recieve);
            loop {
                let mut buf = Vec::new();
                stream.write("hello! 1\x03".as_bytes()).unwrap();
                // stream.write("\x04\x03".as_bytes()).unwrap();
                let mut reader = BufReader::new(&stream);
                reader.read_until(b'\x03', &mut buf).unwrap();
                buf = buf[..(buf.len()-1)].to_vec();
                
                // println!("{:?}", buf)
            }
            
            // break;
        }

        // loop {
        //     for stream in listener.incoming() {
        //         let mut stream = stream.unwrap();
        //         stream.read(&mut buf).unwrap();
        //         println!("{:?}", buf);
        //         // let recieve = stream.read(&mut buf).unwrap();
        //         // println!("{}", recieve);
        //         stream.write("\x04".as_bytes()).unwrap();
        //     }
        // }
    }

    pub fn server(addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        server_service(&listener)

    }
}