use crate::http_request::request_header::*;
use std::collections::HashMap;

use super::extend::HttpResuestExtend;
#[derive(Debug, PartialEq)]
pub struct HttpResuest<'a> {
    method: Method,
    uri: String,
    version: Version,
    headers: HashMap<String, String>,
    body: Option<String>,
    more: HashMap<&'a str, String>,
}

impl<'a> From<String> for HttpResuest<'a> {
    fn from(request: String) -> Self {
        let request_default = HttpResuest::default();
        if request.contains("HTTP/") {
            return HttpResuest::parse_request(&request);
        }
        request_default
    }
}

impl<'a> HttpResuest<'a> {
    fn parse_request(request: &str) -> Self {
        // 空行之后是body内容
        let mut empty_line = false;
        let mut body: Option<String> = None;
        let lines = request.lines().collect::<Vec<&str>>();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut line_iter = lines.iter();
        let protol_header = line_iter.next().unwrap();
        let mut protol_headers = protol_header.split_whitespace();
        let method = protol_headers.next().unwrap();
        let resouse = protol_headers.next().unwrap();
        let version = protol_headers.next().unwrap();
        for line in line_iter {
            if line.is_empty() || line.len() == 0 {
                empty_line = true;
                continue;
            }
            if empty_line {
                body = Some(line.to_string());
                continue;
            }
            let mut headers_iter = line.splitn(2, ':');
            let key = headers_iter.next().unwrap();
            let value = headers_iter.next().unwrap();
            headers.insert(key.to_string(), value.trim().to_string());
        }
        HttpResuest {
            method: method.into(),
            uri: resouse.into(),
            version: version.into(),
            headers: headers,
            body,
            more: HashMap::new(),
        }
    }
}

impl<'a> HttpResuest<'a> {
    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|value| value.as_str())
    }
    pub fn get_uri(&self) -> &str {
        self.uri.as_str()
    }
    pub fn get_header_all(&self) -> &HashMap<String, String> {
        &self.headers
    }
    pub fn get_body(&self) -> Option<&str> {
        self.body.as_ref().map(|value| value.as_str())
    }
}

impl<'a> HttpResuestExtend for HttpResuest<'a> {
    fn set_remote_addr(&mut self, addr: &str) {
        self.more.insert("remote_addr", addr.to_owned());
    }

    fn get_remote_addr(&self) -> String {
        self.more.get("remote_addr").unwrap_or(&"".to_string()).to_string()
    }
}

impl<'a> Default for HttpResuest<'a> {
    fn default() -> Self {
        HttpResuest {
            method: Method::GET,
            uri: "/".to_string(),
            version: Version::V1_1,
            headers: {
                let mut headers: HashMap<String, String> = HashMap::new();
                headers.insert("Content-Type".to_string(), "text/html".to_string());
                headers
            },
            body: None,
            more: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::http_request::{
        request::HttpResuest,
        request_header::{Method, Version},
    };

    #[test]
    fn test_parse_request() {
        let request_str = "GET / HTTP/1.1\r\n";
        let request = HttpResuest::from(request_str.to_string());
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.uri, "/".to_string());
        assert_eq!(request.version, Version::V1_1);
        assert_eq!(request.headers, HashMap::new());
        assert_eq!(request.body, None);
    }

    #[test]
    fn test_parse_request_header_and_body() {
        let request_str = "GET / HTTP/1.1\r\nContent-Type: text/html\r\n\r\nbody";
        let request = HttpResuest::from(request_str.to_string());
        let mut header = HashMap::new();
        header.insert("Content-Type".to_string(), "text/html".to_string());
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.uri, "/".to_string());
        assert_eq!(request.version, Version::V1_1);
        assert_eq!(request.headers, header);
        assert_eq!(request.body, Some("body".to_string()));
    }
}
