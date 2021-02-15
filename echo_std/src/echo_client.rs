use std::io::Read;
use std::thread;
use std::{io::Write, net::TcpStream};

use crate::utils;

pub fn start() {
    let _ = thread::spawn(|| {
        let mut stream = TcpStream::connect("127.0.0.1:1111").unwrap();
        stream.set_nodelay(true).unwrap();
        println!("Connected to {}", &stream.peer_addr().unwrap());

        loop {
            let msg = "hello world";
            let size = msg.len();
            //println!("<- {}", msg);
            stream.write_all(msg.as_bytes()).unwrap();

            let mut buf = vec![0; size];
            stream.read_exact(&mut buf).unwrap();
            utils::count_report::add_count();
            //println!("-> {}\n", String::from_utf8_lossy(&buf));
        }
    });
}
