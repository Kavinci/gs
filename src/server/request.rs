use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub protocol: String,
    pub method: String,
    pub uri: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub message: Body
}

impl Request {
    pub fn new() -> Request {
        Request {
            protocol: String::from(""),
            method: String::from(""),
            uri: String::from(""),
            query: HashMap::new(),
            headers: HashMap::new(),
            message: Body::new()
        }
    }
}

#[derive(Debug)]
pub struct Body {
    pub form: HashMap<String, String>,
    pub file: Vec<u8>,
    pub content: String
}

impl Body {
    pub fn new() -> Body {
        Body {
            form: HashMap::new(),
            file: Vec::new(),
            content: String::from("")
        }
    }
}
