use crate::server::errors::ServerError;
use crate::server::route::Route;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::server::methods::HttpMethod;

use askama::Template;
use crate::models::symbol::Symbol;

#[derive(Template)]
#[template(path = "index.html")]
struct SymbolTemplate {
    symbols: Vec<Symbol>,
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
        let symbols = vec![
            Symbol::new(1, "AAPL".to_string(), 100.0, 0.0, 100.0, 1000, "2020-01-01".to_string(), chrono::Utc::now())
        ];

        if let Some(accept) = req.headers().get("accept") {
            if accept.contains("application/json") {
                return Ok(
                    Response::new(200, "OK")
                )
            }
        }

        let template = SymbolTemplate { symbols };
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