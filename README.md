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

### 待实现功能
- 解析http请求参数
- tcp处理线程池
- 解析请求体
- 文件上传
- 静态资源访问


### 目录模块结构
```
├─http
│  └─src
│      ├─http_handler   # 封装handler相关
│      ├─http_request   # 封装http请求相关
│      └─http_response  # 封装http响应相关
├─simple_http
│  └─src
└─thread_pool           # 封装线程池相关
    └─src
```