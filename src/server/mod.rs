use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::fs;
mod threading;

pub struct HTTP {
    listener: TcpListener
}

impl HTTP {
    pub fn bind(port: String) -> HTTP {
        let address: String = format!("{}:{}", "127.0.0.1", port);
        HTTP {
            listener: TcpListener::bind(address).unwrap()
        }
    }

    pub fn run(&mut self) {
        let pool = threading::ThreadPool::new(16);
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            //handle_connection(stream);
            pool.spawn(|| { handle_connection(stream) });
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;2048];

    stream.read(&mut buffer).unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}