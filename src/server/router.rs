use std::collections::HashMap;
use std::sync::Arc;
use crate::server::errors::ServerError;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::server::route::Route;


#[derive(Default)]
pub struct Router {
    routes: Vec<Arc<dyn Route>>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(&mut self, route: Arc<dyn Route>) {
        self.routes.push(route);
    }

    pub async fn route(&self, req: Request) -> Result<Response, ServerError> {
        for route in &self.routes {
            if route.path_matches(&req.path()) && route.method_matches(&req.method()) {
                return route.handle(req).await;
            }
        }

        let response = Response::new(404, "Not Found");
        Ok(response)
    }
}