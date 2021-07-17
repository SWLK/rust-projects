fn main() {
    // mutable String type allocated to heap
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s);

    // invalidating variable after shallow copying to avoid double free error and memory corruption
    let s1 = String::from("hello");
    let s2 = s1;

    // This line will lead to an error since s1 is invalidated
    // println!("{}, world!", s1);
    
    // Since the first variable is invalidated, it's known as a "move" instead of "shallow copy"
    // pointer, length and capacity are "moved" from stack
    // the data the pointer points to is stored in the heap

    // if a deep copy is intended, we can use the method "clone"
    let s3 = String::from("hello");
    let s4 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // simple scalar values can implement the "copy" trait
    // a type with the "copy" trait cannot have the "drop" trait, and vice versa

    // passing a function will move or copy, just as assignment does
    let str = String::from("hello");    // str in scope

    takes_ownership(str);       // str's value moves into the function, and is no longer valid here

    let x = 5;

    makes_copy(x);      // x is i32, therefore it is copied and still valid here
}

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{}", some_string);
}       // some_string goes out of scope and `drop` is called

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{}", some_integer);
}       // some_integer goes out of scope. Nothing special happens.
