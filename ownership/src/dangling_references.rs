// This code will not compile since the rust compiler will prevent
// the data of a reference from going out of scope before the reference does
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
    
    // This problem could be fixed by returning the String directly
    // s
}
