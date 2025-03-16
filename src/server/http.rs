use std::collections::HashMap;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use std::error::Error;
use std::convert::TryFrom;
use getset::Getters;
use tracing::info;


#[derive(Debug, thiserror::Error)]
pub enum HttpParseError {
    #[error("invalid request format")]
    InvalidFormat,
    #[error("invalid HTTP method: {0}")]
    InvalidMethod(String),
    #[error("invalid request line")]
    InvalidRequestLine,
    #[error("invalid header format")]
    InvalidHeader,
    #[error("missing required headers")]
    MissingRequiredHeaders,
    #[error("error parsing URL: {0}")]
    UrlParseError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl TryFrom<&str> for HttpMethod {
    type Error = HttpParseError;

    fn try_from(method_str: &str) -> Result<Self, Self::Error> {
        match method_str {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(HttpParseError::InvalidMethod(method_str.to_string())),
        }
    }
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct Request {
    method: HttpMethod,
    path: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    pub fn new(
        method: HttpMethod,
        path: String,
        headers: HashMap<String, String>,
        query_params: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Self {
        Self { method, path, headers, query_params, body }
    }
}

impl TryFrom<&str> for Request {
    type Error = HttpParseError;

    fn try_from(request_str: &str) -> Result<Self, Self::Error> {
        let mut lines = request_str.lines();

        // Parse the request line (e.g., "GET /path?query=value HTTP/1.1")
        let request_line = lines.next().ok_or(HttpParseError::InvalidFormat)?;
        let mut parts = request_line.split_whitespace();

        let method_str = parts.next().ok_or(HttpParseError::InvalidRequestLine)?;
        let url_str = parts.next().ok_or(HttpParseError::InvalidRequestLine)?;

        // Parse the HTTP method
        let method = HttpMethod::try_from(method_str)?;

        // Parse the path and query parameters
        let (path, query_params) = parse_url(url_str)?;

        // Parse headers
        let mut headers = HashMap::new();
        let mut blank_line_found = false;

        for line in lines {
            if line.is_empty() {
                blank_line_found = true;
                break;
            }

            let (key, value) = parse_header(line)?;
            headers.insert(key, value);
        }

        if !blank_line_found && !request_str.ends_with("\r\n\r\n") && !request_str.ends_with("\n\n") {
            // No blank line means no body, but that's OK for some methods
            if method == HttpMethod::GET || method == HttpMethod::DELETE {
                return Ok(Request {
                    method,
                    path,
                    headers,
                    query_params,
                    body: Vec::new(),
                });
            }
        }

        // Extract body (everything after the blank line)
        let mut body = Vec::new();
        if blank_line_found {
            let body_start = request_str.find("\r\n\r\n")
                .or_else(|| request_str.find("\n\n"))
                .map(|pos| {
                    if request_str[pos..].starts_with("\r\n\r\n") {
                        pos + 4
                    } else {
                        pos + 2
                    }
                });

            if let Some(start) = body_start {
                if start < request_str.len() {
                    body = request_str[start..].as_bytes().to_vec();
                }
            }
        }

        Ok(Request {
            method,
            path,
            headers,
            query_params,
            body,
        })
    }
}

fn parse_url(url_str: &str) -> Result<(String, HashMap<String, String>), HttpParseError> {
    let mut query_params = HashMap::new();
    let parts: Vec<&str> = url_str.split('?').collect();
    let path = parts[0].to_string();

    if parts.len() > 1 {
        let query_str = parts[1];
        for param in query_str.split('&') {
            let kv: Vec<&str> = param.split('=').collect();
            if kv.len() == 2 {
                query_params.insert(kv[0].to_string(), kv[1].to_string());
            } else if kv.len() == 1 && !kv[0].is_empty() {
                // Query parameter with no value, e.g., "?param"
                query_params.insert(kv[0].to_string(), String::new());
            }
        }
    }

    Ok((path, query_params))
}

fn parse_header(header_line: &str) -> Result<(String, String), HttpParseError> {
    let parts: Vec<&str> = header_line.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(HttpParseError::InvalidHeader);
    }

    let key = parts[0].trim().to_lowercase();
    let value = parts[1].trim().to_string();

    Ok((key, value))
}

#[derive(Debug, Clone)]
pub struct Response {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

pub struct HttpServer {
}

impl HttpServer {
    pub fn new() -> Self {
        Self { }
    }

    pub async fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(&mut stream);

        reader.read_to_end(&mut buffer).await?;
        let request_string = String::from_utf8(buffer)?;

        info!("Received request:\n\n{}", request_string);

        // Parse the request string into a Request object
        let request = Request::try_from(request_string.as_str())?;

        // Now you can handle the parsed request
        info!("Parsed request: {:?}", request);

        // Process the request and generate a response
        // ...

        Ok(())
    }
}