use crate::server::errors::ServerError;
use crate::server::methods::HttpMethod;
use crate::server::request::Request;
use crate::server::response::Response;

#[async_trait::async_trait]
pub trait Route: Send + Sync {
    async fn handle(&self, req: Request) -> Result<Response, ServerError>;
    fn path_matches(&self, path: &str) -> bool;
    fn method_matches(&self, method: &HttpMethod) -> bool;
}