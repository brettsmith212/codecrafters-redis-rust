use std::io::Write;
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
       match stream {
             Ok(mut _stream) => {
              println!("accepted new connection");
               _stream.write(b"+PONG\r\n").unwrap();
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
