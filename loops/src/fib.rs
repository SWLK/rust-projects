use std::io;

fn main() {
    println!("Which iteration of the Fibonacci number?");

    let mut iter = String::new();

    io::stdin()
        .read_line(&mut iter)
        .expect("Failed to read line");

    let iter: u32 = match iter.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut count = 1;

    let mut prev = 0;
    let mut curr = 1;
    let mut next = 0;

    while count <= iter {
        next = prev + curr;

        prev = curr;
        curr = next;

        count += 1;
    }

    println!("The Fibonacci number at iteration {} is {}.", iter, prev);
}
