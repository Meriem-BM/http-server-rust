use std::env;
use std::io::{prelude::*, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client_request(stream));
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }

    fn handle_client_request(mut _stream: TcpStream) -> () {
        let mut buffer = [0; 1024];
        if let Ok(bytes_read) = _stream.read(&mut buffer) {
            if let Ok(request_str) = str::from_utf8(&buffer[0..bytes_read]) {
                if let Some(path) = parse_request_path(request_str) {
                    let req_method = parse_request_method(request_str);
                    let response: String;
                    if path == "/" {
                        response = format!("HTTP/1.1 200 OK\r\n\r\n");
                    } else if path.starts_with("/echo/") {
                        let passed_string: &str = path.trim_start_matches("/echo/");
                        response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                passed_string.len(),
                                passed_string
                            );
                    } else if path == ("/user-agent") {
                        let user_agent: &str = request_str
                            .lines()
                            .find(|line| line.starts_with("User-Agent: "))
                            .unwrap_or("User-Agent: Unknown")
                            .trim_start_matches("User-Agent: ");
                        response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                user_agent.len(),
                                user_agent
                            );
                    } else if path.starts_with("/files/") {
                        let args: Vec<String> = env::args().collect();
                        if (req_method.unwrap() != "GET") {
                            let file_name: &str = path.trim_start_matches("/files/");

                            if args[2].len() < 2 {
                                panic!("Please provide a directory to serve files from");
                            }
                            let directory = if args[2].clone() == "/" || args[2].clone() == "" {
                                env::current_dir().unwrap()
                            } else {
                                env::current_dir().unwrap().join(args[2].clone())
                            };
                            let file_path = directory.join(file_name);

                            if std::path::Path::new(&file_path).exists() {
                                let file = std::fs::read_to_string(file_path).unwrap();
                                response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                                file.len(),
                                file
                            );
                            } else {
                                response = format!(
                                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n"
                            );
                            }
                        } else if req_method.unwrap() == "POST" {
                            // file content from request body
                            let file_content: &str = request_str
                                .lines()
                                .find(|line| line.starts_with("Content-Type: "))
                                .unwrap_or("Content-Type: Unknown")
                                .trim_start_matches("Content-Type: ");

                            let file_name: &str = path.trim_start_matches("/files/");
                            if args[2].len() < 2 {
                                panic!("Please provide a directory to serve files from");
                            }
                            let directory = if args[2].clone() == "/" || args[2].clone() == "" {
                                env::current_dir().unwrap()
                            } else {
                                env::current_dir().unwrap().join(args[2].clone())
                            };
                            let file_path = directory.join(file_name);

                            if std::path::Path::new(&file_path).exists() {
                                response = format!(
                                "HTTP/1.1 409 CONFLICT\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n"
                            );
                            } else {
                                std::fs::write(file_path, file_content).unwrap();
                                response = format!(
                                "HTTP/1.1 201 CREATED\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n"
                            );
                            }
                        } else {
                            response = format!(
                                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n"
                            );
                        }
                    } else {
                        response = format!(
                                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n"
                            );
                    };

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

fn parse_request_path(request: &str) -> Option<&str> {
    let lines: Vec<&str> = request.lines().collect();

    if let Some(start_line) = lines.get(0) {
        let parts: Vec<&str> = start_line.split_whitespace().collect();

        if parts.len() >= 2 {
            return Some(parts[1]);
        }
    }

    None
}

fn parse_request_method(request: &str) -> Option<&str> {
    let lines: Vec<&str> = request.lines().collect();

    if let Some(start_line) = lines.get(0) {
        let parts: Vec<&str> = start_line.split_whitespace().collect();

        if parts.len() >= 1 {
            return Some(parts[0]);
        }
    }

    None
}
