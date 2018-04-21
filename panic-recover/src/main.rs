use std::panic;
fn main() {
    let result = panic::catch_unwind(|| println!("hello!!"));
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| panic!("oh no"));
    assert!(result.is_err());

    // Could reach this code because catch panic usin catch_unwind.
    println!("End of process");
}
