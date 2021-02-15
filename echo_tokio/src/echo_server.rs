use io::{AsyncReadExt, AsyncWriteExt};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

async fn process(mut stream: TcpStream) {
    stream.set_nodelay(true).unwrap();
    let mut buf = vec![0u8; 1024];
    loop {
        let n = stream
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            return;
        }

        stream
            .write_all(&buf[0..n])
            .await
            .expect("failed to write data to socket");
    }
}

pub fn start() {
    let _ = task::spawn(async {
        let listener = TcpListener::bind("0.0.0.0:1111").await.unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());

        while let Ok((stream, _)) = listener.accept().await {
            task::spawn(async {
                process(stream).await;
            });
        }
    });
}
