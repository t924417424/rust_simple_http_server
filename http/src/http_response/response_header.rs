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
    use crate::http_response::{response_header::Version, state_code::HttpStateCode};

    #[test]
    fn version_to_string() {
        assert_eq!(String::from(Version::V1_1), "HTTP/1.1");
        assert_eq!(String::from(Version::V2_0), "HTTP/2.0");
        assert_eq!(String::from(Version::NoSupport), "HTTP/3.0");
    }

    #[test]
    fn http_state_code_to_text() {
        assert_eq!(String::from(HttpStateCode::StatusOK), "OK");
    }

    #[test]
    fn http_state_code_to_u16() {
        assert_eq!(u16::from(HttpStateCode::StatusOK), 200);
    }
}
