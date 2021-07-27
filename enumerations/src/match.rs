enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let penny = Coin::Penny;

    let value = value_in_cents_2(penny);

    println!("The value of the coin is {}.", value);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Curly brackets can be used if there are multiple lines of code in an arm
fn value_in_cents_2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
