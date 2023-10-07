use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // Prepare the HTTP response
                let response = "HTTP/1.1 200 OK\r\n\r\n";

                // Write the response to the client
                if let Err(e) = _stream.write_all(response.as_bytes()) {
                    println!("Error writing to client: {}", e);
                } else {
                    println!("Accepted new connection: {:?}", _stream);
                }
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
