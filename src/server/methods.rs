use crate::server::errors::ServerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl TryFrom<&str> for HttpMethod {
    type Error = ServerError;

    fn try_from(method_str: &str) -> Result<Self, Self::Error> {
        match method_str {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(ServerError::InvalidMethod(method_str.to_string())),
        }
    }
}