// This enum has four variants with different types
enum Message {
    Quit,                           // Quit has no data associated with it
    Move { x: i32, y: i32 },        // Move includes an anonymous struct inside it
    Write(String),                  // Write includes a single `String`
    ChangeColor(i32, i32, i32),     // ChangeColor includes three `i32` values
}

// Defining an enum with variants like this is similar to defining
// different kinds of struct definitions,
// except all the variants are grouped together under the `Message` type.
//
// The following structs could hold the same data that the preceding enum variants hold.
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
};
struct WriteMessage(String);    // tuple struct
struct ChangeColorMessage(i32, i32, i32);   // tuple struct
// But if we use different structs, which each have their own type,
// we couldn't as easily define a function to take any of these kinds of messages
// as we could with the `Message` enum, which is a single type.

// We are also able to define methods on enums.
impl Message {
    fn call(&self) {
        println!("Testing...");
    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));

    m.call();
}
