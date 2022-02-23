use std::fmt::Debug;
use crate::{http_handler::handler::HandlerFn};

use crate::http_method::method::Method;

pub struct RouterHandler {
    pub method: Method,
    pub path: String,
    pub handler: HandlerFn,
}

impl Debug for RouterHandler{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RouterHandler {{ method: {:?}, path: {}, handler: {:p} }}", self.method, self.path, &self.handler)
    }
}

impl RouterHandler {
    pub fn new(method: Method, path: &str, handler: HandlerFn) -> Self {
        RouterHandler {
            method,
            path: path.to_string(),
            handler,
        }
    }
}