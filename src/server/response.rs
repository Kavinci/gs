use std::collections::HashMap;

pub struct Response {
    pub protocol: String,
    pub status_code: usize,
    pub status_message: String,
    pub body: String
}

static CLRF: &str = "\r\n\r\n";
static PROTOCOL: &str = "HTTP/1.1";

impl Response {
    pub fn new() -> Response {
        Response {
            protocol: String::from(PROTOCOL),
            status_code: 200,
            status_message: String::from("OK"),
            body: String::from("")
        }
    }
    pub fn to_string(&mut self) -> String {
        let response: String = format!("{} {} {}{}", PROTOCOL, self.status_code, self.status_message, CLRF);
        response
    }
}

pub struct StatusCode {
    data: HashMap<usize, String>
} 

impl StatusCode {
    pub fn new() -> StatusCode {
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

        StatusCode {
            data
        }
    }

    pub fn get_status(&mut self, code: &usize) -> (bool, &String) {
        let status = self.data.get(code);
        let failure: (bool, &String) = (false, None.unwrap());
        match status {
            Some(String) => (true, status.unwrap()),
            None => failure
        }
    }

    pub fn get_code(&mut self, status: String) {
        let values = self.data.values();
        let mut index: usize = values.len() + 1;
        let max = index.clone();
        for (i, value) in values.enumerate() {
            if *value == status {
                index = i;
            }
        }
        if index < max {

        }
    }
}