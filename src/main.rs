mod resp;

use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;
use resp::Value;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("server ready");

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    handle_conn(stream).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_conn(stream: TcpStream) {
    let mut handler = resp::RespHandler::new(stream);
    println!("starting read loop");
    loop {
        let value = handler.read_value().await.unwrap();
        println!("got value: {:?}", value);

        let response = if let Some(v) = value {
          let (command, args) = extract_command(v).unwrap();
          match command.as_str() {
            "ping" => Value::SimpleString("PONG".to_string()),
            "echo" => args.first().unwrap().clone(),
            c => panic!("Cannot handle command {}", c)
          }
        } else {
          break;
        };

        println!("sending response: {:?}", response);
      
        handler.write_value(response).await.unwrap();
    }
}

fn extract_command(value: Value) -> Result<(String,Vec<Value>)> {
  match value {
    Value::Array(a) => {
      Ok((
        unpack_bulk_str(a.first().unwrap().clone())?,
        a.into_iter().skip(1).collect(),
      ))
    },
    _ => Err(anyhow::anyhow!("Invalid command format")),
  }
}

fn unpack_bulk_str(value: Value) -> Result<String> {
  match value {
    Value::BulkString(s) => Ok(s),
    _ => Err(anyhow::anyhow!("Invalid bulk string format")),
  }
}