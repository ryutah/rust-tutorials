extern crate blog;

use blog::create_state_trait;
use blog::post_with_state;

fn main() {
    with_crate_state_sample();
    with_state_post();
}

fn with_crate_state_sample() {
    let mut post = create_state_trait::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn with_state_post() {
    let mut post = post_with_state::Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
