// Default implementations can call other methods in the same trait, even if those other methods
// don't have a default implementation.
// In this way, a trait can provide a lot of useful functionality and only require implementors to
// specify a small part of it.
// 
// For example, we could define the `Summary` trait to have a `summarize_author` method whose
// implementation is required, and then define a `summarize` method that has a default
// implementation that calls the `summarize_author` method:
pub trait Summary {
    fn sumarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// To use this version of `Summary`, we only need to define `summarize_author` when we implement
// the trait on a type.

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// After we define `summarize_author`, we can call `summarize` on instances of the `Tweet` struct,
// and the default implementation of `summarize will call the definition of `summarize_author` that
// we've provided.
// Because we've implemented `summarize_author, the `Summary` trait has given us the behavior of
// the `summarize` method without requiring us to write any more code.

let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
// This code prints `1 new tweet: (Read more from @horse_ebooks...)`.

