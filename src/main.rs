use std::net::TcpListener;
use std::io::prelude::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                _stream.write(response.as_bytes()).unwrap();
                _stream.flush().unwrap();
                println!("{}", format!("accepted new connection, {:?}", _stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
