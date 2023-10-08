use std::io::{prelude::*, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // Read the request data into a buffer
                let mut buffer = [0; 1024];
                if let Ok(bytes_read) = _stream.read(&mut buffer) {
                    // Convert the bytes to a string
                    if let Ok(request_str) = str::from_utf8(&buffer[0..bytes_read]) {
                        // Parse the request to extract the path
                        if let Some(path) = parse_request_path(request_str) {
                            // Check if the path is of the form "/echo/<a-random-string>"
                            if let Some(random_string) = extract_random_string(&path) {
                                // Prepare the HTTP response with the random string
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
                                    random_string.len(),
                                    random_string
                                );

                                // Write the response to the client
                                if let Err(e) = _stream.write_all(response.as_bytes()) {
                                    println!("Error writing to client: {}", e);
                                } else {
                                    println!("Accepted new connection: {:?}", _stream);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}

fn parse_request_path(request: &str) -> Option<&str> {
    // Split the request into lines
    let lines: Vec<&str> = request.lines().collect();

    // Check if the request has a start line
    if let Some(start_line) = lines.get(0) {
        // Split the start line into parts
        let parts: Vec<&str> = start_line.split_whitespace().collect();

        // Check if there are enough parts
        if parts.len() >= 2 {
            // The path is the second part of the start line
            return Some(parts[1]);
        }
    }

    None
}

fn extract_random_string(path: &str) -> Option<&str> {
    // Check if the path is of the form "/echo/<a-random-string>"
    let mut parts = path.split('/');
    if parts.next() == Some("") && parts.next() == Some("echo") {
        // Extract the random string (the third part)
        if let Some(random_string) = parts.next() {
            return Some(random_string);
        }
    }

    None
}
