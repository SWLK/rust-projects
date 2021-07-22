// A struct's name should describe the significance
// of the pieces of data being grouped together.
// Inside curly brackets, we define the
// names and types of the pieces of data, which we call "fields".
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // To use a struct after defining it,
    // we create an "instance" of that struct by
    // specifying concrete values for each field.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }

    // To get a specific value from a struct,
    // we can use dot notation.
    // If the instance is mutable,
    // we can change a value by using dot notation and assignment.
    user1.email = String::from("anotheremail@example.com");

    // The entire instance must be mutable.
    // Rust doesn't allow marking only certain fields as mutable.

    // Creating a a new instance using values of an old instance
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // We can use the struct update syntax to achieve the same effect
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    // We can construct a new instance of the struct as
    // the last expression in the function to implicitly return that new instance.
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_improved(email: String, username: String) -> User {
    // Since the parameter names and the struct field names are exactly the same,
    // we can use the `field init shorthand` syntax to rewrite build_user
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
