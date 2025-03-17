use std::fs;
use std::path::Path;
use async_trait::async_trait;

use crate::server::errors::ServerError;
use crate::server::route::Route;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::server::methods::HttpMethod;

pub struct StaticFiles {
    base_path: String,
}

impl StaticFiles {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    fn get_content_type(&self, path: &str) -> &str {
        if path.ends_with(".css") {
            "text/css"
        } else if path.ends_with(".js") {
            "application/javascript"
        } else if path.ends_with(".png") {
            "image/png"
        } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
            "image/jpeg"
        } else if path.ends_with(".svg") {
            "image/svg+xml"
        } else {
            "application/octet-stream"
        }
    }
}

#[async_trait]
impl Route for StaticFiles {
    async fn handle(&self, req: Request) -> Result<Response, ServerError> {
        let path = req.path().trim_start_matches('/');
        let file_path = Path::new(&self.base_path).join(path);

        if !file_path.exists() || !file_path.is_file() {
            return Ok(Response::new(404, "Not Found"));
        }

        let content = match fs::read(&file_path) {
            Ok(content) => content,
            Err(_) => return Ok(Response::new(500, "Internal Server Error")),
        };

        let content_type = self.get_content_type(path);

        Ok(Response::new(200, "OK")
            .with_raw_body(content, content_type))
    }

    fn path_matches(&self, path: &str) -> bool {
        path.starts_with("/css/") ||
            path.starts_with("/js/") ||
            path.starts_with("/images/")
    }

    fn method_matches(&self, method: &HttpMethod) -> bool {
        method == &HttpMethod::GET
    }
}