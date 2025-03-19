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
#[template(path = "detail.html")]
struct DetailTemplate {
    symbol: Symbol,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: String,
}

pub struct Detail {
    database: Arc<Database>,
}

impl Detail {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    fn extract_symbol(&self, path: &str) -> Option<String> {
        // Path format should be like "/AAPL" or "/MSFT"
        // Skip the first character (the slash)
        if path.len() > 1 {
            Some(path[1..].to_string())
        } else {
            None
        }
    }
}

#[async_trait::async_trait]
impl Route for Detail {
    async fn handle(&self, req: Request) -> Result<Response, ApplicationError> {
        // Extract the symbol from the path
        let ticker = match self.extract_symbol(req.path()) {
            Some(ticker) => ticker,
            None => {
                let error_template = ErrorTemplate {
                    message: "Invalid symbol format".to_string()
                };
                let html = error_template.render()?;
                return Ok(Response::new(400, "Bad Request").with_html_body(&html));
            }
        };

        // Check if client wants JSON
        if let Some(accept) = req.headers().get("accept") {
            if accept.contains("application/json") {
                // Handle JSON response if needed
                // For now, returning the same error format
                return match self.database.get_symbol_by_ticker(&ticker).await? {
                    Some(_) => Ok(Response::new(200, "OK")),
                    None => Ok(Response::new(404, "Symbol Not Found")),
                };
            }
        }

        // Get the symbol data from the database using the existing method
        match self.database.get_symbol_by_ticker(&ticker).await? {
            Some(symbol) => {
                let template = DetailTemplate { symbol };
                let html = template.render()?;
                Ok(Response::new(200, "OK").with_html_body(&html))
            },
            None => {
                let error_template = ErrorTemplate {
                    message: format!("Symbol '{}' not found", ticker)
                };
                let html = error_template.render()?;
                Ok(Response::new(404, "Not Found").with_html_body(&html))
            }
        }
    }

    fn path_matches(&self, path: &str) -> bool {
        // Match any path with a single segment after the root
        // like "/AAPL" or "/MSFT"
        path.starts_with("/") &&
            path.len() > 1 &&
            !path[1..].contains('/') &&
            path != "/index.html" &&
            !path.starts_with("/css/") &&
            !path.starts_with("/js/") &&
            !path.starts_with("/images/")
    }

    fn method_matches(&self, method: &HttpMethod) -> bool {
        method == &HttpMethod::GET
    }
}