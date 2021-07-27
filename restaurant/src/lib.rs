mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        // `super` is similar to the `..` operator in a file system path
        super::serve_order();
    }

    fn cook_order() {}

    // We can make the struct public with `pub`,
    // but the struct's fields will still be private.
    // We can amke each field public or not on a case-by-case basis.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Because `back_of_house::Breakfast` has a private field,
    // the struct needs to provide a public associated function
    // that constructs an instance of `Breakfast`.
    // If `Breakfast` didn't have such a function,
    // we couldn't create an instance of `Breakfast`
    // because we couldn't set the value of the private `seasonal_fruit` field`.
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In contrast, if we make an enum public, all of its variants are then public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn serve_order() {}

// We can bring a path into a scope once and then call the items in that path
// as if they are local items with the `use` keyword.
use crate::front_of_house::hosting;
// Adding `use` and a path in a scope is similar to creating
// a symbolic link in the filesystem.

// You can also bring an item into scope with `use` and a relative path.
use self::front_of_house::hosting;

// Bringing a function's parent module insto scope with `use`
// so we have to specify the parent module when calling the function
// makes it clear that the function isn't locally defined
// while still minimizing repetition of the full path.

// On the other hand, when bringing in structs, enums, and other items with `use`,
// it's idiomatic to specify the full path.
use std::collections::HashMap;

// When we bring a name into scope with the `use` keyword,
// the name available in the new scope is private.
// To enable the code that calls our code to refer to that name
// as if it had been defined in that code's scope, we can combine `pub` and `use`.
// This technique is called "re-exporting" because we're bringing
// an item into scope but also making that item available for others to bring into their scope.
pub use crate::front_of_house::hosting;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // We have brought the `crate::front_of_house::hosting` module
    // into the scope of the `eat_at_restaurant` function,
    // so we only have to specify `hosting::add_to_waitlist`
    // to call the `add_to_waitlist` function in `eat_at_restaurant`.
    hosting::add_to_waitlist();
}

// The exception to the idiom is if we're bringing two items
// with the same name into scope with `use` statements,
// because Rust doesn't allow that.
// We can do so by bringing in the different parent modules into scope and referring to them.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// We can also specify `as` and a new local name, or alias, for the type.
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
}

fn function4() -> IoResult<()> {
    // --snip--
}

//! Note that the standard library (std) is also a crate that's external to our package.
// Because the standard library is shipped with the Rust language,
// we don't need to change Cargo.toml to include `std`.
// But we do need to refer to it with `use` to bring items from there into our package's scope.

// We can use nested paths to bring the same items into scope in one line.
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};
// We can use a nested path at any level in a path,
// which is useful when combining two `use` statements that share a subpath.
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// If we want to bring all public items defined in a path into scope,
// we can specify that path followed by `*`, the glob operator:
use std::collections::*;