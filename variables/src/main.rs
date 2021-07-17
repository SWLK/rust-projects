fn main() {
    // variable mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    const Y: u32 = 5;
    println!("The value of Y is: {}", Y);

    // variable shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;

    println!("The value of z is: {}", z);

    // shadowing and mutating type
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The number of spaces is: {}", spaces);
}
