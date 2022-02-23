use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    NoSupport,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::NoSupport => write!(f, "NoSupport"),
        }
    }
}

impl From<&str> for Method {
    fn from(method: &str) -> Method {
        match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::NoSupport,
        }
    }
}