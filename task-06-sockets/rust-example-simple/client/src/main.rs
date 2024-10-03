use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:1337") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }

    Ok(())
}