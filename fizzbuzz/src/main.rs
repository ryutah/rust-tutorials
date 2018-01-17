use std::io;
use std::io::Write;
use std::io::BufWriter;

fn main() {
    let cnt = 600000;
    // fizz_buzz_by_match(cnt);
    // fizz_buzz_by_if(cnt);
    fizz_buzz_by_buf(cnt);
}

fn fizz_buzz_by_match(cnt: i32) {
    for i in 0..cnt {
        match i % 15 {
            0 => println!("FizzBuzz"),
            3 => println!("Buzz"),
            5 => println!("Fizz"),
            _ => println!("{}", i),
        };
    }
}

fn fizz_buzz_by_if(cnt: i32) {
    for i in 0..cnt {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn fizz_buzz_by_buf(cnt: i32) {
    let mut w = BufWriter::new(io::stdout());
    for i in 0..cnt {
        match i % 15 {
            0 => writeln!(w, "FizzBuzz"),
            3 => writeln!(w, "Buzz"),
            5 => writeln!(w, "Fizz"),
            _ => writeln!(w, "{}", i),
        };
    }
}
