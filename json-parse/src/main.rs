extern crate serde_json;

use serde_json::{Error, Value};

fn main() {
    let data = r#"{
        "name": "John Due",
        "age": 43,
        "phone": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let v = parse_json(data).expect("failed to parse json");
    println!("Plsease call {} at the number {}", v["name"], v["phone"][0]);
}

fn parse_json(s: &str) -> Result<Value, Error> {
    serde_json::from_str(s)
}
