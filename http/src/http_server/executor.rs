use std::{thread, net::TcpStream};

#[cfg(not(feature = "thread-pool"))]
pub fn executor(_stream: TcpStream) {
    thread::spawn(move || {
        println!("process stream");
    });
    println!("process stream by not thread-pool");
}

#[cfg(feature = "thread-pool")]
pub fn executor(stream: &TcpStream) {
    println!("process stream");
}
