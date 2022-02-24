# 一个Rust实现的简单HttpServer框架

## 个人学习项目，欢迎交流

### 已实现功能
- 解析http请求
    - Post
    - Get
- 响应http请求
    
    [已将Http协议常用状态码封装](./http/src/http_response/state_code.rs)
- Http请求处理逻辑
    - 路由功能
    - 多线处理
        - 1：1多线程（默认方式）
        - 线程池（在cargo中指定features = ["thread-pool"]）


### 待实现功能
- 解析http请求参数
- 路由缓存
- 解析请求体
- 文件上传
- 静态资源访问
- keep-alive


### 目录模块结构
```
├─http
│  └─src
│      ├─http_handler   # 封装handler相关
│      ├─http_request   # 封装http请求相关
│      ├─http_response  # 封装http响应相关
│      ├─http_router    # 路由相关
│      └─http_server    # http服务器参数配置以及启动相关
├─simple_http           # Demo
│  └─src
└─thread_pool           # 封装线程池相关
    └─src
```

### 代码示例
```rust
// main.rs
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
        // 设置线程池中线程的数量,默认为cpu核数+1
        .configure(HttpServer::set_thread_pool_num(10))
        // 设置监听地址，默认为127.0.0.1:8080
        .configure(HttpServer::set_addr("127.0.0.1:8081"))
        // 挂载Router到当前server
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
```

```toml
# Cargo.toml
[package]
name = "simple_http"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
# 如果不指定futures，则默认使用1：1多线程的方式运行
# 当指定features = ["thread-pool"]时，则使用线程池方式运行，线程池中线程数量为CPU核心数+1，也可以通过 HttpServer::set_thread_pool_num(num) 自定义线程池中的线程数量
http = {path = "../http",features = ["thread-pool"]}
```