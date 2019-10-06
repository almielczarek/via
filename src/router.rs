use std::collections::HashMap;

use http::{Method, Request, Response, StatusCode};
use hyper::Body;

type Handler = fn(Request<Body>) -> http::Result<Response<Body>>;

#[derive(Hash, Eq, PartialEq)]
struct Route {
    method: Method,
    path: String,
}

pub struct Router {
    routes: HashMap<Route, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn route(&mut self, method: Method, path: String, handler: Handler) {
        self.routes
            .insert(Route { method, path }, handler);
    }

    pub fn get(&mut self, path: String, handler: Handler) {
        self.route(Method::GET, path, handler);
    }

    pub fn put(&mut self, path: String, handler: Handler) {
        self.route(Method::PUT, path, handler);
    }

    pub fn post(&mut self, path: String, handler: Handler) {
        self.route(Method::POST, path, handler);
    }

    pub fn delete(&mut self, path: String, handler: Handler) {
        self.route(Method::DELETE, path, handler);
    }

    pub fn handle(&self, request: Request<Body>) -> http::Result<Response<Body>> {
        let target = Route {
            method: request.method().clone(),
            path: (*request.uri()).to_string(),
        };

        match self.routes.get(&target) {
            Some(handler) => handler(request),
            None => Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_route() {
        let mut router = Router::new();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/hello")
            .body(Body::empty()).unwrap();

        router.get("/hello".to_owned(), |_request| {
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
        });

        match router.handle(request) {
            Ok(response) => {
                assert_eq!(StatusCode::OK, response.status());
            }
            Err(_) => (),
        };
    }

    #[test]
    fn missing_route() {
        let mut router = Router::new();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/hello")
            .body(Body::empty()).unwrap();

        router.get("/not-found".to_owned(), |_request| {
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
        });

        match router.handle(request) {
            Ok(response) => {
                assert_eq!(StatusCode::NOT_FOUND, response.status());
            }
            Err(_) => (),
        };
    }
}
