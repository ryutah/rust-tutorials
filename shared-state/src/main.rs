use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Simple mutex");
    simple_mutex();

    println!("========================");

    println!("Mutex in thread (Use Arc)");
    thread_mutex();
}

fn simple_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
