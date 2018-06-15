#![feature(mpsc_select)]

use std::io::stdin;
use std::sync::mpsc;
use std::thread;
use std::time;

fn main() {
    let after = time_after(5);
    let input = read_input_async();

    println!("Please input something within 5 seconds passed...");

    select! {
        s = input.recv() => println!("Input is {}", s.unwrap()),
        _ = after.recv() => println!("Time over")
    };
}

fn read_input_async() -> mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        sender.send(s)
    });

    receiver
}

fn time_after(sec: u64) -> mpsc::Receiver<()> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(sec));
        sender.send(()).unwrap();
    });

    receiver
}
