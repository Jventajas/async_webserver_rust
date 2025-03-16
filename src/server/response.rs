use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Response {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}