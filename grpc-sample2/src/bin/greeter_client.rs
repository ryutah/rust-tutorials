extern crate env_logger;
extern crate grpc;
extern crate grpc_sample2;

use grpc_sample2::helloworld::*;
use grpc_sample2::helloworld_grpc::*;

use std::env;

fn main() {
    env_logger::init().expect("env_logger::init");
    let name = env::args()
        .nth(1)
        .map(|s| s.to_owned())
        .unwrap_or_else(|| "world".to_owned());

    let client_conf = Default::default();
    let client = GreeterClient::new_plain("::1", 8080, client_conf).unwrap();

    let mut req = HelloRequest::new();
    req.set_name(name);

    let resp = client.say_hello(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait());
}
