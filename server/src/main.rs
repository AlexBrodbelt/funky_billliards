use std::{env, error::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("failed to process connection; error = {:?}", e);
            }
        });
    }

    Ok(())
}


async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut framed = FramedRead::new(stream, LengthDelimitedCodec::new());
    while let Some(msg) = framed.next().await {
        let msg = msg?;
        let msg = String::from_utf8(msg.to_vec())?;
        println!("Received: {}", msg);
    }

    Ok(())
}
