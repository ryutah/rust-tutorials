extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::{Error, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phone: Vec<String>,
}

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

    let person = parse_json_to_person(data).expect("failed to parse json to person");
    println!("Person is {:?}", person);

    let person_json = person_to_json(&person).expect("failed to marshal to json");
    println!("{}", person_json);
}

fn parse_json(s: &str) -> Result<Value, Error> {
    serde_json::from_str(s)
}

fn parse_json_to_person(s: &str) -> Result<Person, Error> {
    serde_json::from_str(s)
}

fn person_to_json(person: &Person) -> Result<String, Error> {
    serde_json::to_string_pretty(person)
}
