extern crate trait_tutorial;

use std::fmt::Display;
use trait_tutorial::{NewArticle, Summarizable, Tweet};

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());

    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburagh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburagh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summary());

    trait_tutorial::notify(article);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_byclone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}
