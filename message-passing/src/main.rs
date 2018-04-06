use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Simple message");
    simple_message();

    println!("===================================");

    println!("Multiple message");
    multiple_message();

    println!("===================================");

    println!("Multiple cloning message");
    multiple_with_cloning();
}

fn simple_message() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
        // move ownership above code. so this is compile error.
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got! {}", received);
}

fn multiple_message() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_with_cloning() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got! {}", received);
    }
}
