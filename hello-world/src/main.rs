mod foo;
mod bar;

use std::io;

fn main() {
    foo::inside_foo();
    bar::inside_bar();
    println!("{}", foo::get_string());

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error");
    println!("{}", line)
}
