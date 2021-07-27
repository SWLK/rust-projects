#[derive(Debug)]
enum UsState {
    Arkansas,
    Alabama,
    Arizona,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        - => (),
    }

    // This is the same as the match case above.
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // An else block could be included that would be the same as
    // the _ case in `match`.
    let coin = Coin::Quarter(Arkansas);
    let mut count = 0;
    
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
