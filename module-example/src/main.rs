extern crate module_example;

use module_example::{client, network};

fn main() {
    println!("Hello");

    client::connect();
    network::connect();
    network::server::connect();
}
