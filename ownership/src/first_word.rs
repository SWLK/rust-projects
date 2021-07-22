fn main() {
    // the compiler will ensure the references into the String remain valid
    let mut s = String::from("hello world");

    let word = first_word(&s);      // this is a slice of an immutable reference to s

    // since clear truncates the String, it needs a mutable reference to s
    s.clear();      // error

    // the rule holds that there cannot be a mutable reference
    // when there are immutable references present

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {     // &str is the "string slice" type
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];        // slice from start to the first space
        }
    }

    &s[..]
}
