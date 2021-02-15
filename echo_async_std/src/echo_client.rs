use async_std::{io::prelude::ReadExt, io::prelude::WriteExt, net::TcpStream};
//use async_std::prelude::*;
use async_std::task;

use crate::utils;

pub fn start() {
    let _ = task::spawn(async {
        let mut stream = TcpStream::connect("127.0.0.1:1111").await.unwrap();
        stream.set_nodelay(true).unwrap();
        println!("Connected to {}", &stream.peer_addr().unwrap());

        loop {
            let msg = "hello world";
            let size = msg.len();
            //println!("<- {}", msg);
            stream.write_all(msg.as_bytes()).await.unwrap();

            let mut buf = vec![0u8; size];
            stream.read_exact(&mut buf).await.unwrap();
            utils::count_report::add_count();
            //println!("-> {}\n", String::from_utf8_lossy(&buf[.._n]));
        }
    });
}
