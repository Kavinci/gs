use std::io::{prelude::*};
use std::net::{TcpListener, TcpStream};
pub mod threading;
mod request;
mod transient_request;
mod response;

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

    pub fn run(&mut self, handler: fn(TcpStream), pool: threading::ThreadPool) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            pool.spawn(move || { handler(stream) });
        }
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    let res = stream.read(&mut buffer);
    let request: request::Request;
    let transient_request: transient_request::TransientRequest; // = parse_request_transient(buffer); 
    match res {
        Result::Err(_) => transient_request = transient_request::TransientRequest::new(),
        Result::Ok(_) => transient_request = parse_request_transient(&mut buffer), //parse_request(&mut buffer),
    }
    request = parse_request(transient_request);
    let response = handle_request(request).to_string();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request(transient: transient_request::TransientRequest) -> request::Request {
    let mut request = request::Request::new();
    request.parse_request_line(transient.request_line);
    for header in transient.headers {
        request.parse_headers(header);
    }
    request.parse_body(transient.body);
    request
}

fn parse_request_transient(buffer: &mut [u8]) -> transient_request::TransientRequest {
    //println!("{}", String::from_utf8_lossy(&buffer[..]));
    let mut transient = transient_request::TransientRequest::new();
    let mut count: usize = 0;
    let cr = 13u8;
    let lf = 10u8;
    let mut last: u8 = 0;
    let mut now: u8 = 0;
    let mut crlf_count: u8 = 0;
    let mut header: Vec<u8> = Vec::new();
    for buf in buffer {
        last = now.clone();
        now = buf.clone();
        //print!("{} ", &buf);
        // Request Line
        if count == 0 {
            if last == cr && now == lf {
                count = count + 1;
            }
            else {
                transient.request_line.push(now);
            }
        }
        // Headers
        else if count == 1 {
            if last == cr && now == lf {
                crlf_count = crlf_count + 1;
                if crlf_count == 2{
                    count = count + 1;
                }
                else {
                    transient.headers.push(header);
                    header = Vec::new();
                }
            }
            else if now != cr && now != 0u8 {
                //println!("{} : {}", crlf_count, now);
                crlf_count = crlf_count + 1;
            }
            else if  now != 0u8 {
                crlf_count = 0;
                header.push(now);
            }
        }
        // Message Body
        else if count == 2 {
            // ignore the buffer
            if *buf != 0u8 {
                transient.body.push(now);
            }
        }
    }
    transient
}

fn handle_request(request: request::Request) -> response::Response {
    // TODO: parse request and return response
    // Handle headers
    // handle routing
    create_response()
}

fn create_response() -> response::Response {
    let mut response = response::Response::new();
    println!("{}", response.to_string());
    response
}