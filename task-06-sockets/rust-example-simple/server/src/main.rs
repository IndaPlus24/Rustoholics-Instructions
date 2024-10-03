use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("New client: {addr:?}");

            },
            Err(e) => println!("Couldn't get client: {e:?}"),
        };
    }
}


