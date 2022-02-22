use std::net::TcpStream;

pub(crate) trait Executor {
    fn executor(&self, stream: TcpStream);
}
