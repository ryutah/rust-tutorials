extern crate trait_tutorial;

use trait_tutorial::{NewArticle, Summarizable, Tweet};

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
}
