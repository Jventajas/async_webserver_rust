use std::error::Error;
use std::io;
use askama;
use chrono;
use reqwest;
use serde_json;
use sqlx;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ApplicationError {

    #[error("Invalid request format")]
    InvalidRequestFormat,

    #[error("Invalid HTTP method: {0}")]
    InvalidHttpMethod(String),

    #[error("Invalid request line")]
    InvalidRequestLine,

    #[error("Invalid format")]
    InvalidFormat,

    #[error("Invalid header format")]
    InvalidHeaderFormat,

    #[error("Missing required headers")]
    MissingRequiredHeaders,

    #[error("Error parsing URL: {0}")]
    UrlParseError(String),

    #[error("Template rendering error: {0}")]
    TemplateError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Missing database URL")]
    MissingDatabaseUrl,

    #[error("Date parsing error: {0}")]
    DateParseError(#[from] chrono::ParseError),

    #[error("API request failed: {0}")]
    ApiRequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse API response: {0}")]
    ApiResponseParseError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid environment variable: {0}")]
    InvalidEnvVar(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("{0}")]
    OtherError(String),
}

impl From<askama::Error> for ApplicationError {
    fn from(err: askama::Error) -> Self {
        ApplicationError::TemplateError(err.to_string())
    }
}

// impl Error for ApplicationError {}

pub fn to_app_error<E: std::fmt::Display>(err: E) -> ApplicationError {
    ApplicationError::OtherError(err.to_string())
}