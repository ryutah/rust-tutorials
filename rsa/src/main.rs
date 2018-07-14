extern crate openssl;

use openssl::error::ErrorStack;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;

use std::str;

fn main() {
    let key_pair = make_key_pair(2048).unwrap();
    println!("Private key");
    let pem = key_pair.private_key_to_pem_pkcs8().unwrap();
    println!("{}", str::from_utf8(pem.as_slice()).unwrap());

    println!("Public Key");
    let pem = key_pair.public_key_to_pem().unwrap();
    println!("{}", str::from_utf8(pem.as_slice()).unwrap());
}

fn make_key_pair(bit: u32) -> Result<PKey<Private>, ErrorStack> {
    let rsa = Rsa::generate(bit)?;
    PKey::from_rsa(rsa)
}
