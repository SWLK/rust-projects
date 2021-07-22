// Rust includes functionality to print out debugging information,
// but we have to explicitly opt in to make that functionality available.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    }

    // Putting the specifier `:?` inside the curly brackets
    // tells `println!` we want to use an output format called `Debug`.
    // (`:#?` for pretty-print)
    println("rect1 is {:#?}", rect1);
}
