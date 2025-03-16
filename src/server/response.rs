use std::collections::HashMap;
use std::convert::Into;
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum ResponseBody {
    Text(String),
    Json(serde_json::Value),
    Raw(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Response {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: ResponseBody,
}

impl Response {
    pub fn new(status_code: u16, status_text: &str) -> Self {
        Self {
            status_code,
            status_text: status_text.to_string(),
            headers: HashMap::new(),
            body: ResponseBody::Raw(Vec::new()),
        }
    }

    pub fn with_status(mut self, code: u16, text: &str) -> Self {
        self.status_code = code;
        self.status_text = text.to_string();
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_text_body(mut self, text: &str) -> Self {
        self.headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        self.body = ResponseBody::Text(text.to_string());
        self
    }

    pub fn with_html_body(mut self, html: &str) -> Self {
        self.headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
        self.body = ResponseBody::Text(html.to_string());
        self
    }

    pub fn with_json_body<T: Serialize>(mut self, data: &T) -> Result<Self, serde_json::Error> {
        let json_value = serde_json::to_value(data)?;
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.body = ResponseBody::Json(json_value);
        Ok(self)
    }

    pub fn with_raw_body(mut self, body: Vec<u8>, content_type: &str) -> Self {
        self.headers.insert("Content-Type".to_string(), content_type.to_string());
        self.body = ResponseBody::Raw(body);
        self
    }

    fn body_bytes(&self) -> Vec<u8> {
        match &self.body {
            ResponseBody::Text(text) => text.as_bytes().to_vec(),
            ResponseBody::Json(json) => json.to_string().into_bytes(),
            ResponseBody::Raw(bytes) => bytes.clone(),
        }
    }

    fn body_string(&self) -> String {
        match &self.body {
            ResponseBody::Text(text) => text.clone(),
            ResponseBody::Json(json) => json.to_string(),
            ResponseBody::Raw(bytes) => String::from_utf8_lossy(bytes).to_string(),
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        let mut response_str = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);

        for (name, value) in &self.headers {
            response_str.push_str(&format!("{}: {}\r\n", name, value));
        }

        if !self.headers.contains_key("Content-Length") {
            let body_size = match &self.body {
                ResponseBody::Text(text) => text.len(),
                ResponseBody::Json(json) => json.to_string().len(),
                ResponseBody::Raw(bytes) => bytes.len(),
            };
            response_str.push_str(&format!("Content-Length: {}\r\n", body_size));
        }

        response_str.push_str("\r\n");
        response_str.push_str(&self.body_string());
        response_str
    }
}