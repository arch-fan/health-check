use std::{
    env,
    io::Write,
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
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
