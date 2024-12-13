use super::{common::HttpReqRes, status_code::StatusCode};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode) -> Self {
        Self {
            body: status_code.value().0.as_bytes().to_vec(),
            status_code,
            headers: HashMap::new(),
        }
    }
}

impl HttpReqRes for HttpResponse {
    fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    fn deserialize(&self) -> Vec<u8> {
        let mut response_string = String::new();
        let (status_str, status_num) = self.status_code.value();

        // Head insertion
        response_string.push_str(&format!("HTTP/1.1 {} {}\r\n", status_num, status_str));

        // Headers insertion
        for (key, value) in &self.headers {
            response_string.push_str(&format!("{}: {}\r\n", key, value));
        }

        // Empty line to separate headers from body
        response_string.push_str("\r\n");

        // Body insertion
        response_string.push_str(&String::from_utf8_lossy(&self.body));

        response_string.into()
    }

    fn serialize(buff: &Vec<u8>) -> Result<Self, crate::error::HttpSerializeError>
    where
        Self: Sized,
    {
        todo!()
    }
}
