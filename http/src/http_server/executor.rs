use crate::http_response::response::HttpResponse;
use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

#[cfg(not(feature = "thread-pool"))]
pub fn executor(mut stream: TcpStream) {

    thread::spawn(move || {
        let mut req = [0; 512];
        match stream.read(&mut req) {
            Ok(_) => {
                println!("{}", String::from_utf8_lossy(&req));
                let rep: String = HttpResponse::default().into();
                match stream.write_all(rep.as_bytes()) {
                    Ok(_) => {
                        println!("{}", rep);
                    }
                    Err(_) => {
                        println!("write error");
                    }
                }
            }
            Err(_) => {
                println!("read error");
            }
        }
        println!("process stream");
    });
    println!("process stream by not thread-pool");
}

#[cfg(feature = "thread-pool")]
pub fn executor(mut stream: TcpStream) {
    println!("process stream");
}
