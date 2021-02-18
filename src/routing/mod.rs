use crate::server;
use std::collections::HashMap;
use std::fs;
mod parser;

pub fn route(req: &mut server::request::Request, res: &mut server::response::Response) {
    let (path, get_file) = parser::parse_uri(req.uri.clone());
    let mut status = server::response::Status::new();
    if get_file {
        let file = fs::read_to_string(path);
        let (body, headers, status_code) = match file {
            Result::Ok(data) => (data, parser::get_metadata(), 200),
            Result::Err(err) => (format!("{}", err), HashMap::default(), 404),
        };
        res.body = body;
        res.headers.extend(headers);
        status.set_code(status_code);
        res.status = status;
    }
}
