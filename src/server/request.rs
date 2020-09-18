use std::collections::HashMap;

pub struct Request {
    pub protocol: String,
    pub method: String,
    pub uri: String,
    pub query: Query,
    pub headers: Headers,
    pub message: Body
}

impl Request {
    pub fn new() -> Request {
        Request {
            protocol: String::from(""),
            method: String::from(""),
            uri: String::from(""),
            query: Query::new(),
            headers: Headers::new(),
            message: Body::new()
        }
    }

    fn setProtocol(&mut self, protocol: String) {
        self.protocol = protocol;
    }

    fn setMethod(&mut self, method: String){
        self.method = method;
    }

    fn setURI(&mut self, uri: String) {
        let query = Query::new();
        self.setQuery(query);
        self.uri = uri;
    }

    fn addHeader(&mut self, header: String, value: String){
        self.headers.insert(header, value);
    }
    
    fn setQuery(&mut self, query: Query){
        self.query = query;
    }

    pub fn parse_request_line(&mut self, request_line: &mut Vec<u8>) {
        let mut count: u8 = 0;
        let mut component = String::new();
        for buf in request_line {
            if *buf == 32u8{
                if count == 0 {
                    self.setMethod(component);
                }
                else if count == 1 {
                    self.setURI(component);
                }
                else if count == 2 {
                    self.setProtocol(component);
                }
                component = String::new();
                count = count + 1;
            }
            else {
                component.push(*buf as char);
            }
        }
    }

    pub fn parse_headers(&mut self, headers: &mut Vec<u8>) {
        let mut key: String = String::new();
        let mut value: String = String::new();
        let mut count: usize = 0;
        let mut last: u8 = 0;
        let mut now: u8 = 0;
        let mut is_key: bool = true;
        for buf in headers {
            if is_key && last == 58u8 && now == 32u8 {
                is_key = false;
                value.pop();
            }
            if now == 10u8 && last == 13u8 {
                key = key.trim().to_string();
                value = value.trim().to_string();
                if key != "" && value != "" {
                    self.addHeader(key.to_lowercase().clone(), value.clone());
                }
                is_key = true;
                key = String::new();
                value = String::new();
            }
            if now != 0u8 && now != 13u8 && now != 10u8 {
                if is_key && now != 58u8 && now != 32u8 {
                    key.push(now as char);
                }
                else {
                    value.push(now as char);
                }
            }
        }
    }

    pub fn parse_body(&mut self) {}
}

pub type Headers = HashMap<String, String>;
pub type Query = HashMap<String, String>;

pub struct Body {

}

impl Body {
    pub fn new() -> Body {
        Body {}
    }
}