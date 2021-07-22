fn main() {
    let mut s = String::from("Hello World!");

    let word = first_word(&s);      // word will get the value 5

    s.clear();      // this empties the String, making it equal to ""

    // word still has the value 5 but there's no more string to use it with
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();       // converting the String into an array of bytes

    // iter is a method that returns each element in a collection
    // enumerate wraps the result of iter and returns each element as part of a tuple
    // the first element is the index, and the second element is reference to the element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {       // search for the byte that represents the space by using byte literal syntax
            return i;       // if we find a space, return the position
        }
    }

    s.len()     // if no space was found, return the length of the String
}
