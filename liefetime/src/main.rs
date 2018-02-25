use std::fmt::Display;

struct ImportantExcept<'a> {
    part: &'a str,
}

impl<'a> ImportantExcept<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attension please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmeal. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept {
        part: first_sentence,
    };
    println!("Th ImportantExcept : {}", i.part);

    let result = longest_with_an_announcement(string1.as_str(), string2, "Accnoucement");
    println!("The longest string is {}", result);
}

// Lifetime use because arg x and y is valid after return value.
//  ex)
//  let string1 = String::from("abc");
//  let result;
//  {
//      let string2 = String::from("x");
//      result = longest(string1.as_str(), string2.as_str()); // This funciton return string1 or string 2
//  } // string2 will be delete end of scop.
//
//  // result is invalid because it has stirng1 reference or string2 reference and string2 is delete above scope.
//  println!("The longest string is {}", result);
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Accnoucement {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
