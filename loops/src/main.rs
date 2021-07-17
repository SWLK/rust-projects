fn main() {
    // infinite loop
    // loop {
    //     println!("again!");
    // }

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection with for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    
    // for loop with range
    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!");
}
