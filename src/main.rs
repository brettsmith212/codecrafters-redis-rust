use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("server ready");

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((mut stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                  let mut buf = [0; 512];
                  loop {
                    let read_count = stream.read(&mut buf).await.unwrap();
                    if read_count == 0 {
                      break;
                    }
                    stream.write(b"+PING\r\n").await.unwrap();
                  }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
