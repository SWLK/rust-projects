fn main() {
    another_function(5, 6);

    // statements do not return values
    // This does not work
    let x = (let y = 6);

    // expressions evaluate to something
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // functions with return values
    let x = plus_one(5);
    println("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one (x: i32) -> i32 {
    x + 1
}
