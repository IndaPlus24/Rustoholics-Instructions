use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    loop {
        match listener.accept() {
            Ok((_socket, addr)) => println!("new client: {addr:?}"),
            Err(e) => println!("couldn't get client: {e:?}"),
        };
    }
}


