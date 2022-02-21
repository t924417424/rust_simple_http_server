#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    NoSupport,
}

impl From<Version> for String {
    fn from(v: Version) -> Self {
        return match v {
            Version::V1_1 => "HTTP/1.1".to_string(),
            Version::V2_0 => "HTTP/2.0".to_string(),
            Version::NoSupport => "HTTP/3.0".to_string(),
        };
    }
}

// impl Into<&str> for Version {
//     fn into(self) -> &'static str {
//         match self {
//             Version::V1_1 => "HTTP/1.1",
//             Version::V2_0 => "HTTP/2.0",
//             Version::NoSupport => "HTTP/3.0",
//         }
//     }
// }

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

// impl Into<u16> for HttpStateCode {
//     fn into(self) -> u16 {
//         match self {
//             HttpStateCode::OK => 200,
//             HttpStateCode::BadRequest => 400,
//             HttpStateCode::NotFound => 404,
//             HttpStateCode::MethodNotAllowed => 405,
//             HttpStateCode::InternalServerError => 500,
//         }
//     }
// }

// impl Into<&str> for HttpStateCode {
//     fn into(self) -> &'static str {
//         match self {
//             HttpStateCode::OK => "200 OK",
//             HttpStateCode::BadRequest => "400 Bad Request",
//             HttpStateCode::NotFound => "404 Not Found",
//             HttpStateCode::MethodNotAllowed => "405 Method Not Allowed",
//             HttpStateCode::InternalServerError => "500 Internal Server Error",
//         }
//     }
// }

#[cfg(test)]
mod test {
    use crate::http_response::response_header::{HttpStateCode, Version};

    #[test]
    fn version_to_string() {
        assert_eq!(String::from(Version::V1_1), "HTTP/1.1");
        assert_eq!(String::from(Version::V2_0), "HTTP/2.0");
        assert_eq!(String::from(Version::NoSupport), "HTTP/3.0");
    }

    #[test]
    fn http_state_code_to_text() {
        assert_eq!(String::from(HttpStateCode::OK), "OK");
        assert_eq!(String::from(HttpStateCode::BadRequest), "Bad Request");
        assert_eq!(String::from(HttpStateCode::NotFound), "Not Found");
        assert_eq!(
            String::from(HttpStateCode::MethodNotAllowed),
            "Method Not Allowed"
        );
        assert_eq!(
            String::from(HttpStateCode::InternalServerError),
            "Internal Server Error"
        );
    }

    #[test]
    fn http_state_code_to_u16() {
        assert_eq!(u16::from(HttpStateCode::OK), 200);
        assert_eq!(u16::from(HttpStateCode::BadRequest), 400);
        assert_eq!(u16::from(HttpStateCode::NotFound), 404);
        assert_eq!(u16::from(HttpStateCode::MethodNotAllowed), 405);
        assert_eq!(u16::from(HttpStateCode::InternalServerError), 500);
    }
}
