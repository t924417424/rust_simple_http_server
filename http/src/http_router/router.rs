use super::router_handler::RouterHandler;
use crate::http_method::method::Method;
use std::{collections::HashMap, fmt::Debug};
pub struct Router<'a> {
    routes: Vec<Box<RouterHandler>>,
    _routes_cache: HashMap<String, &'a RouterHandler>,
}

impl<'a> Debug for Router<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.routes)
    }
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            _routes_cache: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, handler: RouterHandler) {
        // let paths = Self::split_path(path);
        self.routes.push(Box::new(handler));
    }

    pub(crate) fn get_handler(&self, method: Method, path: &str) -> Option<&RouterHandler> {
        // 首先查询缓存，命中则返回，否则进行匹配
        match self._routes_cache.get(&format!("{}/{}", method, path)) {
            Some(&handler) => Some(handler),
            None => {
                if let Some(router_handler) = self.get_handler_vec(method, path) {
                    return Some(router_handler);
                } else {
                    return None;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn get_handler_vec(&self, method: Method, path: &str) -> Option<&RouterHandler> {
        for route in self.routes.iter() {
            if route.method == method && route.path == path {
                return Some(route);
            }
        }
        None
    }
}
