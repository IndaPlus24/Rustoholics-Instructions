use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    if let Ok(mut socket) = TcpStream::connect("127.0.0.1:1337") {
        println!("Connected to the server!");

        // Send a and b to server
        let (a, b) = (5u8, 6u8);
        socket.write(&[a, b])?;
        socket.flush().expect("Couldn't Flush the message to the socket");
        println!("Send to server: {a}, {b}");

        // Recieve the result
        let mut buf = [0; 1];
        socket.read(&mut buf).expect("Cound't read from the socket");
        let c = &buf[0];
        println!("Received from server: {c}");

    } else {
        println!("Couldn't connect to server...");
        // Works for now, but should be an error
    };

    Ok(())
}