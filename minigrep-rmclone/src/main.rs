extern crate minigrep_rmclone;

use std::env;
use std::process;

use minigrep_rmclone::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_rmclone::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
