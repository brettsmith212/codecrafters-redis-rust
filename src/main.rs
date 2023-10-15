mod internal_state;
mod resp;

use anyhow::Result;
use std::time::{Duration, SystemTime};
use internal_state::{RedisStoredValue, RedisInternalState};
use resp::Value;
use tokio::net::{TcpListener, TcpStream};

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
    let mut internal_state = internal_state::RedisInternalState::new();
    loop {
        let value = handler.read_value().await.unwrap();

        let response = if let Some(v) = value {
            let (command, args) = extract_command(v).unwrap();
            match command.as_str() {
                "ping" | "PING" => Value::SimpleString("PONG".to_string()),
                "echo" | "ECHO" => args.first().unwrap().clone(),
                "get" | "GET" => {
                    println!("redis state: {:?}", internal_state);
                    let key = unpack_bulk_str(args.first().unwrap()).unwrap();
                    let v = format!("Data not available for key: {}", key);
                    let default_value = RedisStoredValue::new(v, None);
                    let stored_value = internal_state.get(&key).unwrap_or(&default_value);
                    Value::SimpleString(stored_value.value().to_string())
                }
                "set" | "SET" => {
                  let output = handle_set(&args, &mut internal_state);
                  match output {
                    Ok(o) => Value::SimpleString(o.to_string()),
                    Err(e) => Value::SimpleString(e.to_string())
                  }
                } 
                c => panic!("Cannot handle command {}", c),
            }
        } else {
            break;
        };

        handler.write_value(response).await.unwrap();
    }
}

fn handle_set(args: &Vec<Value>, internal_state: &mut RedisInternalState) -> Result<String> {
  let key = unpack_bulk_str(&args[0]).unwrap();
  let value = unpack_bulk_str(&args[1]).unwrap();

  let expiration_time_str = match args.get(2) {
    Some(s) => {
      let exp = unpack_bulk_str(s)?.to_uppercase();
      Some(exp)
    },
    None => None,
  };

  let expiration = match expiration_time_str {
    Some(s) if s == "PX" => {
      let time: u64 = unpack_bulk_str(&args[3])?.parse()?;
      let expiration_time = SystemTime::now() + Duration::from_millis(time);
      Some(expiration_time)
    }
    Some(_) => None,
    None => None
  };

  let rsv = RedisStoredValue::new(value, expiration);
  return internal_state.set(&key, &rsv);
}

fn extract_command(value: Value) -> Result<(String, Vec<Value>)> {
    match value {
        Value::Array(a) => Ok((
            unpack_bulk_str(a.first().unwrap())?,
            a.into_iter().skip(1).collect(),
        )),
        _ => Err(anyhow::anyhow!("Invalid command format")),
    }
}

fn unpack_bulk_str(value: &Value) -> Result<String> {
    match value {
        Value::BulkString(s) => Ok(s.to_string()),
        _ => Err(anyhow::anyhow!("Invalid bulk string format")),
    }
}
