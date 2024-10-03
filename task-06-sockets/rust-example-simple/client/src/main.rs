use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:1337") {
        println!("Connected to the server!");
        let (a, b) = (5u8, 6u8);

        stream.write(&[a, b])?;
        stream.flush().expect("Couldn't Flush the message to the stream");
        println!("Send to server: {a}, {b}");
    } else {
        println!("Couldn't connect to server...");
        // Works for now, but should be an error
    }

    Ok(())
}