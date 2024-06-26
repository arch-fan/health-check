use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    println!("Listening on http://0.0.0.0:{port}");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_connection(&mut stream);
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let buffer_to_string = String::from_utf8(buffer.to_vec()).unwrap();
    let parts: Vec<&str> = buffer_to_string.split_whitespace().collect();

    let response = match parts.get(1) {
        Some(path) => match *path {
            "/" => create_res(StatusCode::Ok, "OK"),
            _ => create_res(StatusCode::NotFound, "Not Found"),
        },
        _ => create_res(StatusCode::NotFound, "Not Found"),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

enum StatusCode {
    Ok,
    NotFound,
}

impl StatusCode {
    fn value(&self) -> (u16, &str) {
        match self {
            StatusCode::Ok => (200, "OK"),
            StatusCode::NotFound => (404, "NOT FOUND"),
        }
    }
}

fn create_res(status_code: StatusCode, message: &str) -> String {
    let (status, msg) = status_code.value();
    let message_len = message.len();

    format!("HTTP/1.1 {status} {msg}\r\nContent-Length: {message_len}\r\n\r\n{message}")
}
