use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    let file_name = String::from("foo.txt");
    let ret = read_username_from_file_short(&file_name).expect("Failed to open file");
    println!("{}", ret);
}

fn panic() {
    panic!("crash and burn")
}

fn outof_index() {
    let v = vec![1, 2, 3];
    v[100];
}

fn file_open() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}

fn short_file_open() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file(name: &String) -> io::Result<String> {
    let f = File::open(name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short(name: &String) -> io::Result<String> {
    let mut s = String::new();
    File::open(name)?.read_to_string(&mut s)?;
    Ok(s)
}
