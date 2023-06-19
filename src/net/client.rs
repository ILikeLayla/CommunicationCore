use super::replacer;

pub mod Client {
    pub use std::net::{TcpListener, TcpStream};
    use std::io::{self, prelude::*, BufReader, Write, Error, Read};
    use std::str;
    use std::thread::Thread;
    pub use std::sync::mpsc::{channel, Sender};
    use super::replacer::Replace;

    fn client_service<T: Replace>(stream:TcpStream){
        let (senderLocal, receiverHere) = channel::<T>();
        let mut stream = stream;
        // let (senderHere, recieverLocal) = channel();

        // let mut stream = *stream;

        let mut buff = Vec::new();
        

        loop {
            
            // println!("1");
            // stream.write("hello! 2\x03".as_bytes()).unwrap();
            
            let mut reader = BufReader::new(&stream);
            let mut buf = Vec::new();
            reader.read_until(b'\x03', &mut buf).unwrap();
            // reader.read(&mut buf).unwrap();
            for i in buf.iter() {
                buff.push(*i)
            }

            if buff[buff.len()-2] == b'\x04' {
                buff.clear()
            }
            
            println!("{}", str::from_utf8(&buff).unwrap())
            // println!("2");
            // println!("{:?}", buf);
            // println!("{}", buf.len());

            // if buf.len() != 0 {
            //     // senderHere.send(buf.clone()).unwrap();
            //     // println!("{}", str::from_utf8(&buf[..(buf.len() - 1)]).unwrap());
            //     println!("{:?}", buf);
            // }

            
            
            // if let Ok(a) = receiverHere.try_recv() {
            //     stream.write(a.to_rawdata().as_slice()).unwrap();
            // }
        }
    }

    pub fn client<T: Replace>(addr: &str) {
        let stream = TcpStream::connect(addr).unwrap();
        client_service::<T>(stream);
    }
}