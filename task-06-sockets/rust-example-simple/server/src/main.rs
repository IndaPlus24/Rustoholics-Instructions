use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::path::Component::Prefix;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    loop {
        match listener.accept() {
            Ok((mut socket, addr)) => {
                println!("New client: {addr:?}");
                let mut buf = [0; 2];
                socket.read(&mut buf).expect("Couldn't read from socket");
                println!("From buffer: {buf:?}\n");
            },
            Err(e) => println!("Couldn't get client: {e:?}"),
        };

    }
}


