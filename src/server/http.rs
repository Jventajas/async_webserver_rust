use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use std::error::Error;
use tracing::info;

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

        Ok(())

    }

}