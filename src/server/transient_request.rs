pub struct TransientRequest {
    pub requestLine: Vec<u8>,
    pub headers: Vec<Vec<u8>>,
    pub body: Vec<u8>
}

impl TransientRequest {
    pub fn new() -> TransientRequest {
        TransientRequest{
            requestLine: Vec::new(),
            headers: Vec::new(),
            body: Vec::new()
        }
    }
}