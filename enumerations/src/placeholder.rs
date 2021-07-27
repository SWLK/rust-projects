fn main() {
    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
        // The _ pattern will match any value.
        // The () is just the unit value,
        // so nothing will happen in the _ case.
    };
}
