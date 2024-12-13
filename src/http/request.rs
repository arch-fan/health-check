use super::{common::HttpReqRes, method::HttpMethod};
use crate::error::HttpSerializeError;
use std::{
    collections::HashMap,
    io::{BufRead, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub struct HttpRequest {
    pub headers: HashMap<String, String>,
    pub method: HttpMethod,
    pub path: String,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, path: &str) -> Self {
        Self {
            headers: HashMap::new(),
            method,
            path: path.to_string(),
            body: Vec::with_capacity(1024),
        }
    }
}

impl HttpReqRes for HttpRequest {
    fn serialize(buff: &Vec<u8>) -> Result<HttpRequest, HttpSerializeError> {
        let mut buff_iter = buff.lines();

        let mut headers_res = HashMap::new();
        let method_res: HttpMethod;
        let mut body_res = Vec::with_capacity(1024);
        let path_res: String;

        if let Some(request_line) = buff_iter.next() {
            let request_line = request_line?;
            let mut parts = request_line.split_whitespace();
            if let (Some(method), Some(path)) = (parts.next(), parts.next()) {
                method_res = HttpMethod::from(method);
                path_res = path.to_string();

                while let Some(req_line) = buff_iter.next() {
                    let req_line = req_line?;
                    if req_line.is_empty() {
                        break;
                    }

                    let mut parts = req_line.split(": ");
                    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                        headers_res.insert(key.to_string(), value.to_string());
                    }
                }

                for line in buff_iter {
                    body_res.extend_from_slice(line?.as_bytes());
                }
            } else {
                return Err(HttpSerializeError::new("Failed to parse method or path"));
            }
        } else {
            return Err(HttpSerializeError::new("Failed to parse request line"));
        }

        return Ok(Self {
            method: method_res,
            path: path_res,
            headers: headers_res,
            body: body_res,
        });
    }

    fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    fn deserialize(&self) -> Vec<u8> {
        todo!()
    }
}

impl From<&mut TcpStream> for HttpRequest {
    fn from(stream: &mut TcpStream) -> Self {
        let mut buffer = Vec::with_capacity(1024);
        buffer.resize(1024, 0);
        stream.read(&mut buffer).unwrap();

        HttpRequest::serialize(&buffer).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request() {
        let mut http = HttpRequest::new(HttpMethod::GET, "/");
        http.set_header("Content-Type", "application/json");

        assert_eq!(http.method, HttpMethod::GET);
        assert_eq!(
            http.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert!(http.body.is_empty());
    }

    #[test]
    fn text_http_request_serialize() {
        let request =
            b"POST / HTTP/1.1\r\nContent-Type: application/json\r\nUser-Agent: Firefox\r\n\r\n{\"key\": \"value\"}";
        let http = HttpRequest::serialize(&request.to_vec()).unwrap();

        assert_eq!(http.method, HttpMethod::POST);
        assert_eq!(http.path, "/");
        assert_eq!(
            http.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(http.body, b"{\"key\": \"value\"}");
    }
}
