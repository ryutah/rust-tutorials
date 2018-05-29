extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::Handler;
use std::collections::HashMap;

struct Router {
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route<H>(&mut self, path: String, handler: H)
    where
        H: Handler,
    {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

fn main() {
    routing_sample();
}

fn routing_sample() {
    let mut router = Router::new();

    router.add_route("hello".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello workd!")))
    });

    router.add_route("hello/again".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello again!")))
    });

    router.add_route("error".to_string(), |_: &mut Request| {
        Ok(Response::with(status::BadRequest))
    });

    Iron::new(router).http("localhost:8080").unwrap();
}

fn hello_world() {
    Iron::new(|_: &mut Request| Ok(Response::with((status::Ok, "Hello world"))))
        .http("localhost:8080")
        .unwrap();
}
