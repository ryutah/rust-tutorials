/// say_hello return Hello World String from given arguments
///
/// # Args
/// * name: &str
///   - hello world name.
/// # Returns
/// * String: Hello world string
///
/// # Examples
/// ```
/// let hello = rustdoc::say_hello("John");
/// assert_eq!("Hello, John", hello)
/// ```
pub fn say_hello(name: &str) -> String {
    format!("Hello, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("Hello, John", say_hello("John"));
    }
}
