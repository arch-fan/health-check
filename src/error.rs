use std::error::Error;
use std::fmt;
use std::io;

/// Serialization Error for HttpRequest
#[derive(Debug)]
pub struct HttpSerializeError {
    message: String,
}

impl HttpSerializeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for HttpSerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Error for HttpSerializeError {}

impl From<io::Error> for HttpSerializeError {
    fn from(error: io::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
