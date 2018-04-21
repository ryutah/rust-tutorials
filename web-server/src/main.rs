extern crate futures;
extern crate hyper;

use futures::future::Future;
use futures::Stream;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Body, Chunk};
use hyper::{Method, StatusCode};

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response<Box<Stream<Item = Chunk, Error = Self::Error>>>;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let body: Box<Stream<Item = _, Error = _>> =
                    Box::new(Body::from("Try POSTing to /echo!!"));
                response.set_body(body);
            }
            (&Method::Post, "/echo") => {
                let mapping = req.body().map(to_uppercase as fn(Chunk) -> Chunk);
                let body: Box<Stream<Item = _, Error = _>> = Box::new(mapping);
                response.set_body(body);
            }
            _ => response.set_status(StatusCode::NotFound),
        };

        Box::new(futures::future::ok(response))
    }
}

fn to_uppercase(chunk: Chunk) -> Chunk {
    let uppered: Vec<u8> = chunk.iter().map(|byte| byte.to_ascii_uppercase()).collect();
    Chunk::from(uppered)
}

struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE),
        ))
    }
}

const PHRASE: &str = "Hello World!";

fn main() {
    let adder = "127.0.0.1:8080".parse().unwrap();
    // let server = Http::new().bind(&adder, || Ok(HelloWorld)).unwrap();
    let server = Http::new().bind(&adder, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
