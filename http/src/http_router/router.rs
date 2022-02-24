use super::router_handler::RouterHandler;
use crate::http_method::method::Method;
use std::{collections::HashMap, fmt::Debug};
pub struct Router {
    routes: Vec<Box<RouterHandler>>,
    _routes_cache: HashMap<String, RouterHandler>,
}

impl Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.routes)
    }
}

impl Router {
    /// 创建并返回一个Router实例
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            _routes_cache: HashMap::new(),
        }
    }

    /// 将route注册进Router中
    ///
    /// 1:使用闭包方式注册
    /// ```
    /// let mut router = Router::new();
    /// router.add_route(RouterHandler::new(Method::GET, "/", |_r, w| {
    ///     w.write_str("hello world");
    /// }));
    /// ```
    /// 2:通过函数方式注册
    /// router.add_route(RouterHandler::new(Method::GET, "/hi", route_fn));
    /// fn route_fn(_r: &HttpResuest, w: &mut HttpResponse) {
    /// 输出中文需要添加响应头，否则会出现乱码
    ///     w.insert_header("Content-Type", "text/html;charset=utf-8");
    ///     w.write_str("你好Rust");
    /// }
    pub fn add_route(&mut self, handler: RouterHandler) {
        // let paths = Self::split_path(path);
        self.routes.push(Box::new(handler));
    }

    pub(crate) fn get_handler(&self, method: Method, path: &str) -> Option<&RouterHandler> {
        // 首先查询缓存，命中则返回，否则进行匹配
        match self._routes_cache.get(&format!("{}/{}", method, path)) {
            Some(handler) => Some(handler),
            None => {
                if let Some(router_handler) = self.get_handler_vec(method, path) {
                    return Some(&router_handler);
                } else {
                    return None;
                }
            }
        }
    }

    fn get_handler_vec(&self, method: Method, path: &str) -> Option<&RouterHandler> {
        for route in self.routes.iter() {
            if route.method == method && route.path == path {
                return Some(route);
            }
        }
        None
    }
}
