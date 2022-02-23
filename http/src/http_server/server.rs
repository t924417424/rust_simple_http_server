use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    http_request::{extend::HttpResuestExtend, request},
    http_response::response::HttpResponse,
};

use super::executor::Executor;

#[cfg(not(feature = "thread-pool"))]
#[derive(Debug)]
pub struct HttpServer {
    addr: String,
}

#[cfg(feature = "thread-pool")]
#[derive(Debug)]
pub struct HttpServer {
    addr: String,
    pool: thread_pool::thread_pool::pool::Pool,
}

impl HttpServer {
    /// 返回一个HttpServer实例
    pub fn application() -> Self {
        HttpServer::default()
    }

    /// 设置HttpServer参数
    pub fn configure<F>(&mut self, opt: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        opt(self);
        self
    }

    /// 启动Http服务
    pub fn start(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("http server start at {}", self.addr);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // stream只对当前请求有效，故在此可转移所有权而非借用
                    self.executor(stream);
                }
                Err(_e) => {
                    continue;
                }
            }
        }
    }
}

impl HttpServer {
    /// 设置HttpServer监听地址，默认值："127.0.0.1:8080" 
    pub fn set_addr(addr: &str) -> impl FnOnce(&mut Self) {
        // 不可直接捕获参数所有权
        let a = addr.to_owned();
        |t: &mut Self| {
            t.addr = a;
        }
    }

    /// 设置线程池大小，默认线程数：cpu核数 + 1
    pub fn set_thread_pool_num(num: usize) -> impl FnOnce(&mut Self) {
        let n = num;
        // 加入Move强制转移所有权，否则n的生命周期不够长
        move |t: &mut Self| {
            t.pool = thread_pool::thread_pool::pool::Pool::new(n);
        }
    }
}

#[cfg(not(feature = "thread-pool"))]
impl Executor for HttpServer {
    fn executor(&self, stream: TcpStream) {
        use std::thread;

        thread::spawn(move || {
            // println!("process stream");
            Self::parse_stream(stream);
        });
        println!("process stream by not thread-pool");
    }
}

#[cfg(feature = "thread-pool")]
impl Executor for HttpServer {
    fn executor(&self, stream: TcpStream) {
        // println!("process stream");
        self.pool.execute(move || {
            Self::parse_stream(stream);
        });
    }
}

impl HttpServer {
    fn parse_stream(mut stream: TcpStream) {
        let mut buf: Vec<u8> = Vec::new();
        let _len = Self::parse_stream_to_request(&mut stream, &mut buf);
        let mut request =
            request::HttpResuest::from(String::from_utf8_lossy(buf.as_slice()).to_string());
        request.set_remote_addr(stream.peer_addr().unwrap().to_string().as_str());
        let rep: String = HttpResponse::default().into();
        if let Err(e) = stream.write_all(rep.as_bytes()) {
            println!("response write error: {}", e);
        }
    }

    // 读取http请求信息，这个函数可能存在一些bug，比如读取到的数据不完整，需要继续读取
    fn parse_stream_to_request(stream: &mut TcpStream, buf: &mut Vec<u8>) -> usize {
        let mut req = [0; 1024];
        let mut pack_len: usize = 0;
        // println!("remote ip : {}", stream.peer_addr().unwrap().to_string());
        while let Ok(len) = stream.read(&mut req) {
            // println!("pack len : {}", len);
            if len == 0 {
                break;
            }
            pack_len += len;
            buf.extend_from_slice(&req[..len]);
            if len < req.len() {
                break;
            }
        }
        pack_len
    }
}

#[cfg(not(feature = "thread-pool"))]
impl Default for HttpServer {
    fn default() -> Self {
        HttpServer {
            addr: "127.0.0.1:8080".parse().unwrap(),
        }
    }
}

#[cfg(feature = "thread-pool")]
impl Default for HttpServer {
    fn default() -> Self {
        let cpu_num = num_cpus::get();
        HttpServer {
            addr: "127.0.0.1:8080".parse().unwrap(),
            pool: thread_pool::thread_pool::pool::Pool::new(cpu_num + 1),
        }
    }
}
