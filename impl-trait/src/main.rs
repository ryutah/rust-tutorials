trait MyTrait {
    fn print(&self);
}

impl MyTrait for i32 {
    fn print(&self) {
        println!("This is i32 function: {}", self);
    }
}

impl MyTrait for u32 {
    fn print(&self) {
        println!("This is u32 function: {}", self);
    }
}

fn main() {
    let foo = i32_my_trait();
    foo.print();

    let foo = u32_my_trait();
    foo.print();
}

fn i32_my_trait() -> impl MyTrait {
    return 1 as i32;
}

fn u32_my_trait() -> impl MyTrait {
    return 2 as u32;
}
