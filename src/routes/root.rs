use std::sync::Arc;
use askama::Template;

use crate::server::route::Route;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::server::methods::HttpMethod;
use crate::models::symbol::Symbol;
use crate::services::database::Database;
use crate::utils::error::ApplicationError;

#[derive(Template)]
#[template(path = "index.html")]
struct SymbolTemplate {
    symbols: Vec<Symbol>,
}

pub struct Root {
    database: Arc<Database>,
}

impl Root {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}


#[async_trait::async_trait]
impl Route for Root {
    async fn handle(&self, req: Request) -> Result<Response, ApplicationError> {
        let symbols = self.database.get_all_symbols().await?;

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