use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};
use std::{io::Write, thread};

fn process(mut stream: TcpStream) {
    stream.set_nodelay(true).unwrap();
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                // write data
                if let Err(e) = stream.write_all(&buf[0..n]) {
                    eprintln!("failed to write to the stream; err = {:?}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("failed to read from the stream; err = {:?}", e);
                break;
            }
        }
    }
}

pub fn start() {
    let _ = thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:1111").unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next() {
            let stream = stream.unwrap();
            thread::spawn(|| {
                process(stream);
            });
        }
    });
}
