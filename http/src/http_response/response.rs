use crate::http_response::response_header::Version;
use std::collections::HashMap;

use super::state_code::HttpStateCode;
#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    // connection: &'a mut TcpStream,
    pub version: Version,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

/// 为HttpResponse加入默认实现
impl Default for HttpResponse {
    fn default() -> Self {
        HttpResponse {
            version: Version::V1_1,
            status_code: HttpStateCode::StatusNotFound.into(),
            headers: {
                let mut header = HashMap::new();
                header.insert("Content-Type".to_string(), "text/html".to_string());
                header
            },
            body: None,
        }
    }
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse::default()
    }
    pub fn set_http_state_code(&mut self, status_code: HttpStateCode) -> &mut Self {
        self.status_code = status_code.into();
        self
    }
    pub fn write_str(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.to_string());
        self
    }
    pub fn insert_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}

impl From<HttpResponse> for String {
    fn from(http_response: HttpResponse) -> Self {
        let http_code: u16 = http_response.status_code;
        let tmp: HttpStateCode = http_response.status_code.into();
        let code_text: String = tmp.into();
        let mut response_str = format!(
            "{} {} {}\r\n",
            String::from(http_response.version),
            http_code,
            code_text
        );
        for (key, value) in http_response.headers.iter() {
            response_str.push_str(&format!("{}: {}\r\n", key, value));
        }
        let content_length = http_response
            .body
            .as_ref()
            .map(|value| value.len())
            .unwrap_or(code_text.len());
        response_str.push_str(&format!("Content-Length: {}\r\n", content_length));
        response_str.push_str("\r\n");
        if let Some(body) = http_response.body {
            response_str.push_str(&body);
        } else {
            response_str.push_str(&code_text);
        }
        return response_str;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_http_response_from_str() {
        use super::HttpStateCode;
        let mut response = super::HttpResponse::new();
        response.insert_header("Content-Type", "text/html");
        response.set_http_state_code(HttpStateCode::StatusOK);
        response.write_str("<html></html>");
        let response_str: String = response.into();
        assert_eq!(
            response_str,
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\n<html></html>"
        );
    }
}
