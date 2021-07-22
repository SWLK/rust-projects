// There can only be one mutable reference
// to a particular piece of data in a particular scope.
// This prevents data races at compile time.
// A data race occurs when two or more pointers access the same data at the same time,
// and there's no mechanism is there to synchronize access to the data.

fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    // There also cannot be a mutable reference while there's an immutable one
    let r1 = &s;        // no problem
    let r2 = &s;        // no problem
    let r3 = &mut s;    // problem

    // println!("{}, {}, and {}", r1, r2, r3);
    // There can be multiple immutable references since they are undergoing read operations only

    // A reference's scope begins when it is introduced and ends on last usage
    let r4 = &s;
    let r5 = &s;

    println!("{} and {}", r4, r5);

    let r6 = &mut s;
    println!("{}", r6);
    // This works since the immutable references' scope has ended before the mutable reference was
    // introduced
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

