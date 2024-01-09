use std::{error::Error, io};

use tokio::net::{TcpListener, TcpStream};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let addr = ("127.0.0.1", args.port);
    let listener = TcpListener::bind(addr).await?;

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("Client connected from {:?}", addr);
                tokio::spawn(async move { process_socket(socket).await });
            },
            Err(e) => println!("Could not get client: {:?}", e),
        }
    }

    Ok(())
}

/// yields when client disconnects
async fn process_socket(socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_reader(socket);

    loop {
        socket.readable().await?;



        let mut buf = [0; 4096];

        match socket.try_read(&mut buf) {
            Ok(0) => break Ok(()),
            Ok(n) => {

            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }
    }
}
