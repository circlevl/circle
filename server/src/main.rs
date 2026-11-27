mod error;
mod model;

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), error::CustomError> {
    let listener = TcpListener::bind("127.0.0.1:5050").await?;
    println!("circle-server stared!");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("addr: {:?}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_stream(stream).await {
                eprintln!("handle_stream error: {:?}", e);
            }
        });
    }
}

async fn handle_stream(stream: TcpStream) -> Result<(), error::CustomError> {
    let (mut reader, _) = stream.into_split();
    loop {
        let mut payload = [0u8; 4];

        match reader.read_exact(&mut payload).await {
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    return Err(error::CustomError::IO(e));
                }
            }

            Ok(_) => {
                let payload_size: usize = u32::from_be_bytes(payload) as usize;
                let mut message_vec = vec![0u8; payload_size];
                reader.read_exact(&mut message_vec).await?;

                let message: model::Message = wincode::deserialize(&message_vec)?;
                println!("message: {:?}", message_vec);
            }
        }
    }
}
