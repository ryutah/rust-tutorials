struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // Rust automatically call this method when free CustomSmartPointer goes out of scope.
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    println!("Hello, world!");
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.");
    // It is not allowd to call `drop` by yourself.
    // c.drop(); // This cause compile error!
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
