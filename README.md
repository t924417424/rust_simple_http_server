# 一个Rust实现的简单HttpServer框架

## 个人学习项目，欢迎交流

### 已实现功能
- 解析http请求
    - Post
    - Get
- 响应http请求
    - 响应200
    - 响应400
    - 响应404
    - 响应405
    - 响应500
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
- 路由功能


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