// Now we can explore how to use traits to define functions that accept many different types.
// We implemented the `Summary` trait on the `NewsArticle` and `Tweet` types.

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {}

// We can define a `notify` function that calls the `summarize` method on its `item` parameter,
// which is of some type that implements the `Summary` trait.
// To do this, we can use the `impl Trait` syntax:
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the trait
// name.
// This parameter accepts any type that implements the specified trait.
// In the body of `notify`, we can call any methods on `item` that come from the `Summary` trait,
// such as `summarize`.
// We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`.
// Code that calls the function with any other type, such as a `String` or an `i32`, won't compile
// because those types don't implement `Summary`.
//
// ====================================================================
//
// Trait Bound Syntax
// 
// The `impl Trait` syntax works for straitforward cases but it is actually syntax sugar for a
// longer form, which is called a trait bound.
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// This longer form is equivalent to the example in the previous section but is more verbose.
// We place trait bounds with the declaration of the generic type parameter after a colon and
// inside angle brackets.
//
// The `impl Trait` syntax is convenient and makes for more concise code in simple cases.
// The trait bound syntax can express more complexity in other cases.
// For example, we can have two parameters that implement `Summary:
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// If we wanted this function to allow `item1` and `item2` to have different types
