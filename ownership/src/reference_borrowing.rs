fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);    // &s1 refers to the value of s1 but does not own it

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // the signature uses `&` to indicate the parameter is a reference
    s.len()
}   // s goes out of scope, but since it does not have ownership, nothing gets dropped

// Since references are borrowed, they are immutable
