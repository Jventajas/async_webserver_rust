#[derive(Debug, thiserror::Error)]
pub enum ServerError {
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
    #[error("template error: {0}")]
    TemplateError(String)
}

impl From<askama::Error> for ServerError {
    fn from(err: askama::Error) -> Self {
        ServerError::TemplateError(err.to_string())
    }
}
