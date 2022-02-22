use crate::{http_request::request, http_response::response};

pub type HandlerFn = fn(request: &request::HttpResuest, response: &mut response::HttpResponse);
