pub struct TransientRequest {
    pub request_line: Vec<u8>,
    pub headers: Vec<Vec<u8>>,
    pub body: Vec<u8>
}

impl TransientRequest {
    pub fn new() -> TransientRequest {
        TransientRequest{
            request_line: Vec::new(),
            headers: Vec::new(),
            body: Vec::new()
        }
    }
}