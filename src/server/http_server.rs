use std::collections::HashMap;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use std::error::Error;
use std::convert::TryFrom;
use getset::Getters;
use tracing::info;

use crate::server::errors::HttpParseError;
use crate::server::http_methods::HttpMethod;
use crate::server::request::Request;

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
        let request = Request::try_from(request_string.as_str())?;

        info!("Parsed request: \n\n{:?}", request);






        // Process the request and generate a response
        // ...

        Ok(())
    }
}