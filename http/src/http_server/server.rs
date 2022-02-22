use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct HttpServer {
    addr: String,
}

impl HttpServer {
    pub fn application() -> Self {
        HttpServer::default()
    }

    pub fn configure<F>(&mut self, opt: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        opt(self);
        self
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("http server start at {}", self.addr);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // stream只对当前请求有效，故在此可转移所有权而非借用
                    Self::process_stream(stream);
                }
                Err(_e) => {
                    continue;
                }
            }
        }
    }

    #[cfg(not(feature = "thread-pool"))]
    fn process_stream(_stream: TcpStream) {
        use std::thread;
        thread::spawn(|| {
            println!("process stream");
        });
        println!("process stream by not thread-pool");
    }

    #[cfg(feature = "thread-pool")]
    fn process_stream(stream: &TcpStream) {
        println!("process stream");
    }
}

impl HttpServer {
    pub fn set_addr(addr: &str) -> impl FnOnce(&mut Self) {
        // 不可直接捕获参数所有权
        let a = addr.to_owned();
        |t: &mut Self| {
            t.addr = a;
        }
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        HttpServer {
            addr: "127.0.0.1:8080".parse().unwrap(),
        }
    }
}
