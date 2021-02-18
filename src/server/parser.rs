use crate::server::request;
use std;

const CR: u8 = '\r' as u8;
//const LF: u8 = '\n' as u8;
const SPACE: u8 = ' ' as u8;
const QMARK: u8 = '?' as u8;
const EQUALS: u8 = '=' as u8;
const AMP: u8 = '&' as u8;
const COLON: u8 = ':' as u8;

pub struct Parser<I: Iterator<Item = u8>> {
    iter: std::iter::Peekable<I>,
    request: request::Request,
}

impl<I: Iterator<Item = u8>> Parser<I> {
    pub fn new(iter: I) -> Self {
        Parser {
            iter: iter.peekable(),
            request: request::Request::new(),
        }
    }

    pub fn parse(mut self) -> request::Request {
        self.request_line();
        self.headers();
        self.body();
        self.request
    }
    fn request_line(&mut self) {
        self.method();
        self.target();
        self.version();
        self.iter.next();
    }

    fn method(&mut self) {
        loop {
            match (self.iter.next(), self.iter.peek()) {
                (Some(c), Some(&SPACE)) => {
                    self.request.method.push(c as char);
                    self.iter.next();
                    break;
                }
                (Some(c), _) => {
                    self.request.method.push(c as char);
                }
                _ => panic!("method parsing error"), //TODO: handle error for parsing state
            }
        }
    }

    fn target(&mut self) {
        loop {
            match (self.iter.next(), self.iter.peek()) {
                (Some(c), Some(&QMARK)) => {
                    self.request.uri.push(c as char);
                    self.iter.next();
                    self.query_string();
                    break;
                }
                (Some(c), Some(&SPACE)) => {
                    self.request.uri.push(c as char);
                    self.iter.next();
                    break;
                }
                (Some(c), _) => {
                    self.request.uri.push(c as char);
                }
                _ => panic!("uri parsing error"), //TODO: handle error for parsing state
            }
        }
    }

    fn query_string(&mut self) {
        loop {
            let (key, end) = self.get_query_key();
            if end {
                self.request.query.insert(key, String::from(""));
                break;
            }
            let (value, end) = self.get_query_value();
            self.request.query.insert(key, value);
            if end {
                break;
            }
        }
    }

    fn get_query_key(&mut self) -> (String, bool) {
        let mut key: String = String::from("");
        loop {
            match (self.iter.next(), self.iter.peek()) {
                (Some(c), Some(&SPACE)) => {
                    key.push(c as char);
                    self.iter.next();
                    return (key, true);
                }
                (Some(c), Some(&EQUALS)) => {
                    key.push(c as char);
                    self.iter.next();
                    return (key, false);
                }
                (Some(c), _) => {
                    key.push(c as char);
                }
                _ => panic!("query parsing error - key"), //TODO: handle error for parsing state
            }
        }
    }

    fn get_query_value(&mut self) -> (String, bool) {
        let mut value: String = String::from("");
        loop {
            match (self.iter.next(), self.iter.peek()) {
                (Some(c), Some(&AMP)) => {
                    value.push(c as char);
                    self.iter.next();
                    return (value, false);
                }
                (Some(SPACE), _) => return (String::from(""), true),
                (Some(c), Some(&SPACE)) => {
                    value.push(c as char);
                    self.iter.next();
                    return (value, true);
                }
                (Some(c), _) => {
                    value.push(c as char);
                }
                _ => panic!("query parsing error - value"), //TODO: handle error for parsing state
            }
        }
    }

    fn version(&mut self) {
        loop {
            match (self.iter.next(), self.iter.peek()) {
                (Some(c), Some(&CR)) => {
                    self.request.protocol.push(c as char);
                    self.iter.next();
                    self.iter.next();
                    break;
                }
                (Some(c), _) => {
                    self.request.protocol.push(c as char);
                }
                (None, _) => panic!("error on version parser"),
            }
        }
    }

    fn headers(&mut self) {
        loop {
            let (key, end) = self.get_header_key();
            if end {
                self.request.headers.insert(key, String::from(""));
                break;
            }
            let (value, mut end) = self.get_header_value();
            if self.iter.peek() == Some(&CR) {
                self.iter.next(); // consume CR
                self.iter.next(); // consume LF
                end = true;
            }
            self.request.headers.insert(key, value);
            if end {
                break;
            }
        }
    }

    fn get_header_key(&mut self) -> (String, bool) {
        let mut key: String = String::from("");
        loop {
            match (self.iter.next(), self.iter.peek()) {
                (Some(c), Some(&COLON)) => {
                    key.push(c as char);
                    self.iter.next(); // consume colon
                    self.iter.next(); // consume space
                    return (key, false);
                }
                (Some(c), _) => {
                    key.push(c as char);
                }
                _ => panic!("query parsing error - key"), //TODO: handle error for parsing state
            }
        }
    }

    fn get_header_value(&mut self) -> (String, bool) {
        let mut value: String = String::from("");
        loop {
            match (self.iter.next(), self.iter.peek()) {
                (Some(c), Some(&CR)) => {
                    value.push(c as char);
                    self.iter.next(); // consume CR
                    self.iter.next(); // consume LF
                    return (value, false);
                }
                (Some(c), _) => {
                    value.push(c as char);
                }
                _ => panic!("query parsing error - value"), //TODO: handle error for parsing state
            }
        }
    }

    fn body(&self) {}
}
