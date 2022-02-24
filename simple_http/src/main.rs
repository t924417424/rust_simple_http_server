use http::{
    http_method::method::Method,
    http_request::request::HttpResuest,
    http_response::response::HttpResponse,
    http_router::{router::Router, router_handler::RouterHandler},
    http_server::server::HttpServer,
};
fn main() {
    // 初始化一个Application
    let mut server = HttpServer::application();
    // 创建一个Router
    let mut router = Router::new();
    // 将route函数添加到router中
    router.add_route(RouterHandler::new(Method::GET, "/", |_r, w| {
        w.write_str("hello world");
    }));
    router.add_route(RouterHandler::new(Method::GET, "/hi", route_fn));
    // 设置Http监听地址，并挂载Router到该服务中
    server
        .configure(HttpServer::set_addr("127.0.0.1:8081"))
        .mount_route(router);
    // 启动Http服务
    server.start();
    println!("{:?}", server);
}

fn route_fn(_r: &HttpResuest, w: &mut HttpResponse) {
    // 输出中文需要添加响应头，否则会出现乱码
    w.insert_header("Content-Type", "text/html;charset=utf-8");
    println!("remote addr: {}", _r.get_remote_addr());
    if let Some(header) = _r.get_header("User-Agent") {
        println!("User-Agent: {}", header);
    }
    w.write_str("你好Rust");
}
