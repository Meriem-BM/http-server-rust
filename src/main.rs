use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("{}", format!("accepted new connection",));
                write!(&mut stream, "HTTP/1.1 200 OK\r\n\r\n ").expect("Failed to respond.");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
