use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::error::Error;
use std::convert::TryFrom;
use std::sync::Arc;
use getset::Getters;
use tracing::info;

use crate::server::errors::ServerError;
use crate::server::methods::HttpMethod;
use crate::server::request::Request;
use crate::server::router::Router;
use crate::server::route::Route;
use crate::routes::root::Root;

pub struct HttpServer {
    router: Router,
}

impl HttpServer {
    pub fn new() -> Self {
        let mut routes: Vec<Arc<dyn Route>> = Vec::new();
        let root = Arc::new(Root::new());
        routes.push(root);

        let mut router = Router::new(routes);



        Self {
            router,
        }
    }

    pub async fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(&mut stream);

        reader.read_to_end(&mut buffer).await?;
        let request_string = String::from_utf8(buffer)?;
        let request = Request::try_from(request_string.as_str())?;

        info!("Parsed request: \n\n{:?}", request);

        let response = self.router.route(request).await?;
        let response_str: String = response.into();

        info!("Response: \n\n{:?}", response_str);

        stream.write(response_str.as_bytes()).await?;




        // Process the request and generate a response
        // ...

        Ok(())
    }
}