fn main() {
    // a string slice is a reference to part of a String
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    // [starting_index..ending_index]
    // where starting index is the first position in the slice
    // and ending index is one more than the last position in the slice

    // internally, the slice data structure stores
    // the starting position and the length of the slice,
    // which corresponds to ending_index minus starting_index

    // if you want to start at the first index (zero), you can drop the starting value
    let slice_from_start = &s[..2];

    // similarly, if the slice includes the last byte of the String,
    // you can drop the trailing number
    let slice_to_end = &s[3..];

    // You can drop both values to slice the entire string
    let slice_whole = &s[..];
}
