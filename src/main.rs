#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::env;

fn main() {
    println!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    let directory = args.iter().position(|arg| arg == "--directory")
        .and_then(|index| args.get(index + 1))
        .map(|s| s.to_string());
    
    if let Some(dir) = &directory {
        println!("Serving files from directory: {}", dir);
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let directory_clone = directory.clone();
                std::thread::spawn(move || {
                println!("accepted new connection");

                let mut buffer = [0; 1024];
                let bytes_read = stream.read(&mut buffer).unwrap();

                let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                println!("Request: {}", request);
                
                if request.starts_with("GET /files/") {

                    if let Some(dir) = directory_clone {

                        let request_line = request.lines().next().unwrap();
                        
                        let parts: Vec<&str> = request_line.split(' ').collect();
                        let path = parts[1]; 
                        let filename = &path[7..];
                        
                        let file_path = format!("{}/{}", dir, filename);
                        println!("Looking for file: {}", file_path);
                        
                        match fs::read(&file_path) {
                            Ok(content) => {
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                                    content.len()
                                );
                                stream.write(response.as_bytes()).unwrap();
                                stream.write(&content).unwrap();
                            },
                            Err(_) => {
                                println!("File not found: {}", file_path);
                                stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
                            }
                        }
                    } else {
                        stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
                    }
                }
                else if request.starts_with("POST /files/"){
                    let request_line = request.lines().next().unwrap();
                    let parts: Vec<&str> = request_line.split(' ').collect();
                    let path = parts[1]; 
                    let filename = &path[7..];
                    
                    let file_path = format!("{}/{}", directory_clone.unwrap(), filename);
                    println!("Writing to file: {}", file_path);
                    
                    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
                    fs::write(&file_path, body).unwrap();
                    
                    let response = "HTTP/1.1 201 Created\r\n\r\n";
                    stream.write(response.as_bytes()).unwrap();
                }
                else if request.starts_with("GET /user-agent"){
                    let user_agent = request.lines().find(|line| line.starts_with("User-Agent:")).unwrap_or("");
                    let response_body = user_agent.split_whitespace().nth(1).unwrap_or("");
                    let response_body = response_body.to_string();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        response_body.len(),
                        response_body
                    );
                    stream.write(response.as_bytes()).unwrap();
                }
                else if request.starts_with("GET /echo/"){
                    let path = &request[10..];
                    let extracted_str = path.split_whitespace().next().unwrap();
                    let response_body = extracted_str.to_string();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        extracted_str.len(),
                        response_body
                    );
                    stream.write(response.as_bytes()).unwrap();
                }
                else if request.starts_with("GET /") && !request.starts_with("GET / HTTP/1.1") {
                    stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
                } 
                else {
                    stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
                }
            });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}