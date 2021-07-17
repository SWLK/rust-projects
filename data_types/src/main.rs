fn main() {
    // integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // booleans
    let t = true;
    let f: bool = false;

    // char
    let c = 'z';

    // primitive compound types
    //
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);    // explicit type declaration
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;    // destructuring
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    // arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];  // explicit type and length declaration
    let a = [3; 5];     // alternative initialization [value; length]
    let first = a[0];
}
