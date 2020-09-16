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

    pub fn setProtocol(&mut self, protocol: String) {
        self.protocol = protocol;
    }

    pub fn setMethod(&mut self, method: String){
        self.method = method;
    }

    pub fn setURI(&mut self, uri: String) {
        let query = Query::new();
        self.setQuery(query);
        self.uri = uri;
    }
    
    pub fn setQuery(&mut self, query: Query){
        self.query = query;
    }
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