mod error;
mod model;

use std::io;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), error::CustomError> {
    let mut stream = TcpStream::connect("127.0.0.1:5050").await?;
    println!("stream connected!");

    let mut username = String::new();
    let mut roomname = String::new();
    let mut command = String::new();
    let mut userid = String::new();

    println!("enter username:");
    io::stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();

    println!("enter roomname:");
    io::stdin().read_line(&mut roomname).unwrap();
    roomname = roomname.trim().to_string();

    println!("enter command (create or join):");
    io::stdin().read_line(&mut command).unwrap();
    command = command.trim().to_string();

    println!("enter userid:");
    io::stdin().read_line(&mut userid).unwrap();
    userid = userid.trim().to_string();

    let message = model::Message {
        username,
        roomname,
        command: if command == "create" {
            model::Commands::Create
        } else {
            model::Commands::Join
        },
        userid,
    };

    let message_serialized = wincode::serialize(&message)?;
    let message_size = message_serialized.len() as u32;

    let mut message_vec = Vec::new();
    message_vec.extend_from_slice(&message_size.to_be_bytes());
    message_vec.extend_from_slice(&message_serialized);

    stream.write_all(&message_vec).await?;
    Ok(())
}
