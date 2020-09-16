use std::io::{prelude::*, IoSliceMut};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::fs;
use io;
mod threading;
mod request;
mod transient_request;

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
            pool.spawn(|| { handle_connection(stream) });
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    let res = stream.read(&mut buffer);
    let response: &str;
    let request: request::Request; 
    match res {
        Result::Err(_) => request = request::Request::new(),
        Result::Ok(_) => request = parse_request(&mut buffer),
    }
    response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request(buffer: &mut [u8]) -> request::Request {
    let mut request = request::Request::new();
    let mut key: String = String::new();
    let mut value: String = String::new();
    let mut count: usize = 0;
    let mut last: u8 = 0;
    let mut now: u8 = 0;
    let mut is_key: bool = true;
    let mut pairs: usize = 0;
    // for buf in buffer {
    //     last = now.clone();
    //     now = buf.clone();
    //     print!("{} ", &buf);
    //     if count == 0 {
    //         if now == 32u8{
    //             //println!("0: {}", value);
    //             request.insert(String::from("method"), value.clone());
    //             count = count + 1;
    //             value = String::new();
    //         } 
    //         else {
    //             value.push(now as char);
    //         }
    //     }
    //     else if count == 1 {
    //         if now == 32u8{
    //             //println!("1: {}", value);
    //             request.insert(String::from("identifier"), value.clone());
    //             count = count + 1;
    //             value = String::new();
    //         }
    //         else {
    //             value.push(now as char);
    //         }
    //     }
    //     else if count == 2 {
    //         if now == 13u8 {
    //             //println!("2: {}", &value);
    //             request.insert(String::from("protocol"), value.clone());
    //             count = count + 1;
    //             value = String::new();
    //         }
    //         else {
    //             value.push(now as char);
    //         }
    //     }
    //     else if count == 3 {
    //         if is_key && last == 58u8 && now == 32u8 {
    //             is_key = false;
    //             value.pop();
    //         }
    //         if now == 10u8 && last == 13u8 {
    //             key = key.trim().to_string();
    //             value = value.trim().to_string();
    //             if key != "" && value != "" {
    //                 request.insert(key.to_lowercase().clone(), value.clone());
    //             }
    //             is_key = true;
    //             key = String::new();
    //             value = String::new();
    //         }
    //         if now != 0u8 && now != 13u8 && now != 10u8 {
    //             if is_key && now != 58u8 && now != 32u8 {
    //                 key.push(now as char);
    //             }
    //             else {
    //                 value.push(now as char);
    //             }
    //         }
    //     }
    // }
    request::Request::new()
}

fn parse_request_transient(buffer: &mut [u8]) -> transient_request::TransientRequest {
    println!("{}", String::from_utf8_lossy(&buffer[..]));
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
                transient.requestLine.push(*buf);
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
            else if now != cr {
                crlf_count = crlf_count - 1;
            }
            else {
                header.push(*buf);
            }
        }
        // Message Body
        else if count == 2 {
            // ignore the buffer
            if *buf != 0u8 {
                transient.body.push(*buf);
            }
        }
    }
    transient
}

fn parse_request_line(request_line: &mut Vec<u8>, request: &mut request::Request) -> request::Request {
    let mut count: u8 = 0;
    let mut component = String::new();
    for buf in request_line {
        if *buf == 32u8{
            if count == 0 {
                request.setMethod(component);
            }
            else if count == 1 {
                request.setURI(component);
            }
            else if count == 2 {
                request.setProtocol(component);
            }
            component = String::new();
            count = count + 1;
        }
        else {
            component.push(*buf as char);
        }
    }
    *request
}

fn parse_headers() {}

fn parse_body() {}