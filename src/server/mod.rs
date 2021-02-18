extern crate ctrlc;
use crate::configuration;
use crate::routing;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod parser;
pub mod request;
pub mod response;
pub mod threading;

pub struct HTTP {
    listener: TcpListener,
    terminate: bool,
}

impl HTTP {
    pub fn bind(config: configuration::Configuration) -> HTTP {
        let address: String = format!("{}:{}", "127.0.0.1", config.port.clone());
        HTTP {
            listener: TcpListener::bind(address).unwrap(),
            terminate: false,
        }
    }

    pub fn run(&mut self, handler: fn(TcpStream), pool: &threading::ThreadPool) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            pool.spawn(move || {
                handler(stream);
            });
            if self.terminate {
                break;
            }
        }
    }

    // pub fn kill(&mut self){
    //     self.terminate = true;
    // }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //TODO: Dynamic buffer, possibly inspect TCP headers
    let res = stream.read(&mut buffer);
    let (request, response) = match res {
        Result::Err(_) => (request::Request::new(), response::Response::new()),
        Result::Ok(_) => parse_request(&mut buffer),
    };
    let mut response = handle_request(request, response);
    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request(buffer: &mut [u8]) -> (request::Request, response::Response) {
    //println!("{}", String::from_utf8_lossy(&buffer[..]));
    let iter = buffer.iter().copied();
    let parser = parser::Parser::new(iter);
    (parser.parse(), response::Response::new())
}

fn handle_request(
    mut request: request::Request,
    mut response: response::Response,
) -> response::Response {
    // TODO: Handle headers
    routing::route(&mut request, &mut response);
    print!("{:#?}\n{:#?}\n", request, response);
    response
}
