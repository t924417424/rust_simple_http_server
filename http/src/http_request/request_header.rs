// PartialEq:方便在Test中进行对比,否则assert_eq将不能对该类型进行断言对比
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    NoSupport,
}

impl From<&str> for Version {
    fn from(version: &str) -> Version {
        match version {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::NoSupport,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    NoSupport,
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

#[derive(Debug, PartialEq)]
pub enum Resouse {
    Path(String),
}

impl From<&str> for Resouse {
    fn from(resouse: &str) -> Resouse {
        Resouse::Path(resouse.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_version() {
        assert_eq!(Version::from("HTTP/1.1"), Version::V1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::V2_0);
        assert_eq!(Version::from("HTTP/3.0"), Version::NoSupport);
    }

    #[test]
    fn test_method() {
        assert_eq!(Method::from("GET"), Method::GET);
        assert_eq!(Method::from("POST"), Method::POST);
        assert_eq!(Method::from("DELETE"), Method::NoSupport);
    }

    #[test]
    fn test_resouse() {
        assert_eq!(Resouse::from("/"), Resouse::Path("/".to_string()));
        assert_eq!(
            Resouse::from("/index.html"),
            Resouse::Path("/index.html".to_string())
        );
    }
}
