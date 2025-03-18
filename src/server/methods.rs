use crate::utils::error::ApplicationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl TryFrom<&str> for HttpMethod {
    type Error = ApplicationError;

    fn try_from(method_str: &str) -> Result<Self, Self::Error> {
        match method_str {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(ApplicationError::InvalidHttpMethod(method_str.to_string())),
        }
    }
}