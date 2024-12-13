pub mod error;
pub mod http;
use std::thread;
use std::{env, io::Write, net::TcpListener};

use http::common::HttpReqRes;
use http::method::HttpMethod;
use http::request::HttpRequest;
use http::response::HttpResponse;
use http::status_code::StatusCode;

const IP: &str = "127.0.0.1";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    println!("Listening on http://{IP}:{port}");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            thread::spawn(move || {
                let res = handle_connection(HttpRequest::from(&mut stream));

                if stream.write(res.deserialize().as_slice()).is_err() {
                    println!("Failed to write to stream");
                }

                if stream.flush().is_err() {
                    println!("Failed to flush stream");
                }
            });
        }
    }

    Ok(())
}

fn handle_connection(req: HttpRequest) -> HttpResponse {
    println!("Request path: {}", req.path);
    if req.method == HttpMethod::GET && req.path == "/" {
        HttpResponse::new(StatusCode::Ok)
    } else {
        HttpResponse::new(StatusCode::NotFound)
    }
}
