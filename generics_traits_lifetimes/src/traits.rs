// Here, we declare a trait using the `trait` keyword and then the trait's name, which is `Summary`
// in this case.
// Inside the curly brackets, we declare the method signatures that describe the behaviors of the
// types that implement this trait, which in this case is `fn summarize(&self) -> String`.
pub trait Summary {
    fn summarize(&self) -> String;
}
// After the method signature, instead of providing an implementation within curly brackets, we use
// a semicolon.
// Each type implementing this trait must provide its own custom behavior for the body of the
// method.
// The compiler will enforce that any type that has the `Summary` trait will have the method
// `summarize` defined with this signature exactly.
//
// A trait can have multiple methods in its body: the method signatures are listed one per line and
// each line ends in a semicolon.

// ==============================================================
// Now that we've defined the desired behavior using the `Summary` trait, we can implement it on
// the types in our media aggregator.
// We can create an implementation of the `Summary` trait on the `NewsArticle` struct that uses the
// headline, the author, and the location to create the return value of `summarize`.
// For the `Tweet` struct, we define `summarize` as the username followed by the entire text of the
// tweet, assuming that tweet content is already limited to 280 characters.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Implementing a trait on a type is similar to implementing regular methods.
// The difference is that after `impl`, we put the trait name that we want to implement, then use
// the `for` keyword, and then specify the name of the type we want to implement the trait for.
// Within the `impl` block, we put the method signatures that the trait definition has defined.
// Instead of adding a semicolon after each signature, we use curly brackets and fill in the method
// body with the specific behavior that we want the methods of the trait to have for the particular
// type.
//
// After implementing the trait, we can call the methods on instaces of `NewsArticle` and `Tweet`
// in the same way we call regular methods.
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet .summarize());
}

// Because we defined the `Summary` trait and the `NewsArticle` and `Tweet` types in the same file,
// they're all in the same scope.
// Let's say this file is for a crate we've called aggregator and someone else wants to use our
// crate's functionality to implement the `Summary` trait on a struct defined within their
// library's scope. They would need to bring the trait into their scope first.
// They would do so by specifying `use aggregator::Summary;`, which then would enable them to
// implement `Summary` for their type.
// =================================================================

// One restriction to note with trait implementations is that we can implement a trait on a type
// only if either the trait or the type is local to our crate.
