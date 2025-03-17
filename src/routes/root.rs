use crate::server::errors::ServerError;
use crate::server::route::Route;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::server::methods::HttpMethod;
use crate::models::index::Index;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    indexes: Vec<Index>,
}

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
        let indexes = vec![Index::new(1, "SYMBOL".to_string(), "Sample Name".to_string(), 100.0, 1.5, chrono::Utc::now())
        ];

        if let Some(accept) = req.headers().get("accept") {
            if accept.contains("application/json") {
                return Ok(
                    Response::new(200, "OK")
                )
            }
        }

        let template = IndexTemplate { indexes };
        let html = template.render()?;

        Ok(
            Response::new(200, "OK")
                .with_html_body(&html)
        )

    }

    fn path_matches(&self, path: &str) -> bool {
        path == "/" || path == "/index.html"
    }

    fn method_matches(&self, method: &HttpMethod) -> bool {
        method == &HttpMethod::GET
    }
}