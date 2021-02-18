use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub protocol: String,
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: String,
}

static CLRF: &str = "\r\n";
static PROTOCOL: &str = "HTTP/1.1";

impl Response {
    pub fn new() -> Response {
        let mut header_list = HashMap::new();
        header_list.insert("Server".to_owned(), "General Store".to_owned());
        Response {
            protocol: String::from(PROTOCOL),
            status: Status::new(),
            headers: header_list,
            body: String::from(""),
        }
    }
    pub fn to_string(&mut self) -> String {
        let headers = self.parse_headers();
        format!(
            "{} {} {}{}{}{}{}",
            PROTOCOL, self.status.code, self.status.message, CLRF, headers, CLRF, self.body
        )
    }
    fn parse_headers(&mut self) -> String {
        let mut headers: String = String::default();
        for (key, val) in self.headers.iter() {
            headers = headers + key + ": " + val + CLRF;
        }
        headers = headers + "Date: " + &Utc::now().to_rfc2822() + CLRF;
        headers
    }
}
#[derive(Debug)]
pub struct Status {
    code: usize,
    message: String,
    data: HashMap<usize, String>,
}

impl Status {
    pub fn new() -> Status {
        let mut data: HashMap<usize, String> = HashMap::new();
        data.insert(100, "Continue".to_owned());
        data.insert(101, "Switching Protocols".to_owned());
        data.insert(200, "OK".to_owned());
        data.insert(201, "Created".to_owned());
        data.insert(202, "Accepted".to_owned());
        data.insert(204, "No Content".to_owned());
        data.insert(400, "Bad Request".to_owned());
        data.insert(401, "Unauthorized".to_owned());
        data.insert(403, "Forbidden".to_owned());
        data.insert(404, "Not Found".to_owned());
        data.insert(500, "Internal Server Error".to_owned());
        data.insert(501, "Not Implemented".to_owned());
        data.insert(502, "Bad Gateway".to_owned());
        data.insert(503, "Service Unavailable".to_owned());
        data.insert(504, "Gateway Time-out".to_owned());
        data.insert(505, "HTTP Version not supported".to_owned());

        Status {
            code: 200,
            message: "OK".to_owned(),
            data,
        }
    }
    pub fn set_message(&mut self, message: String) {
        self.message = message.clone();
        self.code = *self.get_code(&message);
    }
    pub fn set_code(&mut self, code: usize) {
        self.code = code.clone();
        self.message = self.get_message(&code);
    }
    fn get_message(&mut self, code: &usize) -> String {
        let mut status: String = String::default();
        for (key, val) in self.data.iter() {
            if key == code {
                status = val.to_string();
            }
        }
        status
    }
    fn get_code(&mut self, status: &String) -> &usize {
        let mut code: &usize = &200;
        for (key, val) in self.data.iter() {
            if val == status {
                code = key;
            }
        }
        code
    }
}
