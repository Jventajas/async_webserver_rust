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