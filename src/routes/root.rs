use crate::server::errors::ServerError;
use crate::server::route::Route;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::server::methods::HttpMethod;


pub struct Root {

}

impl Root {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait::async_trait]
impl Route for Root {
    async fn handle(&self, req: Request) -> Result<Response, ServerError> {
        if let Some(accept) = req.headers().get("accept") {
            if accept.contains("application/json") {
                return Ok(
                    Response::new(200, "OK")
                        .with_header("Content-Type", "application/json")
                )
            }
        }

        Ok(
            Response::new(200, "OK")
                .with_header("Content-Type", "text/plain; charset=utf-8")
                .with_text_body("You did it!!")
        )








    }

    fn path_matches(&self, path: &str) -> bool {
        path == "/" || path == "/index.html"
    }

    fn method_matches(&self, method: &HttpMethod) -> bool {
        method == &HttpMethod::GET
    }
}