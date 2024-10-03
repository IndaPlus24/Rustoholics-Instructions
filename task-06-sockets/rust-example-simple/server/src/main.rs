use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::path::Component::Prefix;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    loop {
        match listener.accept() {
            Ok((mut socket, addr)) => {
                println!("New client: {addr:?}");

                // Read from socket to buffer
                let mut buf = [0; 2];
                socket.read(&mut buf).expect("Couldn't read from socket");

                // Retrieve data
                let (a, b) = (&buf[0], &buf[1]);
                let c = a + b;
                println!("From buffer: {buf:?}");
                println!("{a} + {b} = {c}\n");
            },
            Err(e) => println!("Couldn't get client: {e:?}"),
        };

    }
}


