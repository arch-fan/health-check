#[derive(Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn from(value: &str) -> Self {
        match value {
            "OK" => StatusCode::Ok,
            "Bad Request" => StatusCode::BadRequest,
            "Not Found" => StatusCode::NotFound,
            "Internal Server Error" => StatusCode::InternalServerError,
            _ => StatusCode::InternalServerError,
        }
    }

    pub fn value(&self) -> (&str, u16) {
        match self {
            StatusCode::Ok => ("OK", 200),
            StatusCode::BadRequest => ("Bad Request", 400),
            StatusCode::NotFound => ("Not Found", 404),
            StatusCode::InternalServerError => ("Internal Server Error", 500),
        }
    }
}
