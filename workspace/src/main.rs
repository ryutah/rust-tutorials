extern crate add_one;

fn main() {
    println!("Hello, world!");
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}",
        num,
        add_one::add_one(num)
    );
}
