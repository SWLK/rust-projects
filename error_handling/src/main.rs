fn main() {
    // When you're writing an example to illustrate some concept,
    // having robust error-handling code in the example as well can make the example less clear.
    // In examples, it's understood that a call to a method like `unwrap` that could panic
    // is meant as a placeholder for the way you'd want your application to handle errors.

    // It would also be appropriate to call `unwrap` when you have some other logic
    // that ensures the `Result` will have an `Ok` value, but the logic isn't something the compiler understands.
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1".parse().unwrap();
    // We're creating an `IpAddr` instance by parsing a hardcoded string.
    // We can see that `127.0.0.1` is a valid IP address, so it's acceptable to use `unwrap` here.
    // However, having a hardcoded, valid string doesn't change the return type of the `parse` method:
    // we still get a `Result` value, and the compiler will still make us handle the `Result` as if
    // the `Err` variant is a possibility because the compiler isn't smart enough to see that
    // this tring is always a valid IP address.

    // It's advisable to have your code panic when it's possible that your code could end up in a bad state.
    // A "bad state" is when some assumption, guarantee, contract, or invariant has been broken, such as
    // invalid values, contradictory values, or missing values are passed to your code, plus the following:
    // + The bad state is not something that's expected to happen occassionally
    // + Your code after this point needs to rely on not being in this bad state
    // + There's not a good way to encode this information in the types you use

    // Attempting to operate on invalid data can expose your code to vulnerabilities.
    // This is the main reason the standard library will call `panic!` if you attempt an
    // out-of-bounds memory access: trying to access memory that doesn't belong to the current
    // data structure is a common security problem

    // Attempting to operate on invalid data can expose your code to vulnerabilities.
    // This is the main reason the standard library will call `pani!c!` if you attempt an
    // out-of-bounds memory access: trying to access memory that doesn't belong to the current
    // data structure is a common security problem.
    // Functions often have contracts: their behavior is only guaranteed if the inputs meet particular requirements.
    // Panicking when the contract is violated makes sense because a contract violation always indicates
    // a caller-side gu and it's not a kind of error you want the calling code to have to explicitly handle.
}

fn custom_types() {
    // In Chapter 2 our code asked the user to guess a number between 1 and 100.
    // We never validated that the user's guess was between those numbers before
    // checking it against our secret number; we only validated that the guess was positive (type checking).
    // It would be a useful enhancement to guide the user toward valid guesses and have
    // different behavior when a user guesses a number that's out of range versus when a user types,
    // for example, letters instead.

   // One way to do this would be to parse the guess as an `i32` instead of only a `u32`
   // to allow potentially negative numbers, and then add a check for the number being in range:
   use std::io;
   use rand::Rng;
   use std::cmp::Ordering;

   println!("Guess the number!");

   let secret_number = rand::thread_rng().gen_range(1..101);

   loop {
       println!("Please input your guess.");

       let mut guess = String::new();

       io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Attempt to parse the input to an i32, continue if parsing failed (char or strings)
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Check if the number is in range
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
   }
}

fn custom_types_2() {
    // However, this is not an ideal solution:
    // if it was absolutely critical that the program only operated on values between 1 and 100,
    // and it had many functions with this requirement, having a check like this in every function
    // would be tedious (and might impact performance).

    // Instead, we can make a new type and put the validations in a function to create an
    // instance of the type rather than repeating the validations everywhere.
    // That way, it's safe for functions to use the new type in their signatures and confidently
    // use the values they receive.
    struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {value}
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
    // We implement an associated function named `new` on `Guess` that creates instances of `Guess` values.
    // The code in the body of the `new` function tests `value` to amke sure it's between 1 and 100.
    // If `value` doesn't pass this test, we make a `panic!` call, which will alert the programmer
    // who is writing the calling code that they have a bug they need to fix, because creating a `Guess`
    // with a value outside this range would violate the contract that `Guess::new` is relying on.
    // The conditions in which `Guess:new` might panic should be discussed in its public-facing API documentation.

    // Next, we implement a method named `value` that borrows `self`, doesn't have any other parameters,
    // and returns an `i32`.
    // This kind of method is sometimes called a "getter", because its purpose is to get some data from its fields.
    // This public method is necessary because the `value` field of the `Guess` struct is private.
    // It's important that the `value` field be private so code using the `Guess= struct
    // is not allowed to set `value` directly: code outside the module must use the `Guess::new` function
    // to create an instance of `Guess`, thereby ensuring there's no way for a `Guess` to have a value
    // that hasn't been checked by the conditions in the `Guess::new` function.
}