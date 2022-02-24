#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HttpStateCode {
    OK,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
}

impl From<HttpStateCode> for u16 {
    fn from(state_code: HttpStateCode) -> Self {
        return match state_code {
            HttpStateCode::OK => 200,
            HttpStateCode::BadRequest => 400,
            HttpStateCode::NotFound => 404,
            HttpStateCode::MethodNotAllowed => 405,
            HttpStateCode::InternalServerError => 500,
        };
    }
}

impl From<HttpStateCode> for String {
    fn from(state_code: HttpStateCode) -> Self {
        return match state_code {
            HttpStateCode::OK => "OK".to_string(),
            HttpStateCode::BadRequest => "Bad Request".to_string(),
            HttpStateCode::NotFound => "Not Found".to_string(),
            HttpStateCode::MethodNotAllowed => "Method Not Allowed".to_string(),
            HttpStateCode::InternalServerError => "Internal Server Error".to_string(),
        };
    }
}