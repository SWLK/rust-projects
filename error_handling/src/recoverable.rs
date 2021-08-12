pub fn main() {
	// Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.
	//The `Result` enum is defined as having two variants, `Ok` and `Err`.
	enum Result<T, E> {
		Ok(T),
		Err(E),
	}
	// The `T` and `E` are generic type parameters.
	// `T` represents the type of the value that will be returned in a success case within the `Ok` variant,
	// and `E` represents the type of the error that will be returned in a failure case within the `Err` variant.
	// Because `Result` has these generic type parameters, we can use the `Result` type
	// and the functions that the standard library has defined on it in many different situations
	// where the successful value and error value we want to return may differ.

	use std::fs::File;

	let f: u32 = File::open("hello.txt");
	// If we give `f` a type annotation that we know is not the return type of the function
	// and then try to compile the code, the compiler will tell us that the types don't match.
	// The error message will then tell us what the type of `f` is.

	// The message will tell us that the return type of the `File::open` function is a `Result<T, E>`.
	// The generic parameter `T` has been filled in with the type of the success value,
	// `std::fs::File`, which is a file handle.
	// The type of `E` used in the error value is `std::io::Error`.

	// The `File::open` function needs to have a way to tell us whether it succeeded or failed
	// and at the same time give us either the file handle or error information.
	// This information is exactly what the `Result` enum conveys.

	// We need to add to the code to take different actions depending on the value `File::open` returns.
	let f = File::open("hello.txt");

	let f = match f {
		Ok(file) => file,
		Err(error) => panic!("Problem opening the file: {:?}", error),
	};
	// Like the `Option` enum, the `Result` enum and its variants have been brought into scope by the prelude,
	// so we don't need to specify `Result::` before the `Ok` and `Err` variants in the `match` arms.
	// When the result is `Ok`, return the inner `file` value out of the `Ok` variant,
	// and we then assign that file handle value to the variable `f`.
	// If we get an `Err` value from `File::open`, we've chosen to call the `panic!` macro.
}

fn matching_different_errors() {
	// We can take different actions for different failure reasons:
	// If `File::open` failed because the file doesn't exist,
	// we want to create the file and return the handle to the new file.
	// If `File::open` failed for any other reason - for example, because we don't have permission to open the file,
	// - we still want the code to `panic!`.
	use std::fs::File;
	use std::io::ErrorKind;

	let f = File::open("hello.txt");

	let f = match f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {:?}", e),
			},
			other_error => {
				panic!("Problem opening the file: {:?}", other_error)
			},
		},
	};
	// The type of value that `File::open` returns inside the `Err` variant is `io::Error`,
	// which is a struct provided by the standard library.
	// This struct has a method `kind` that we can call to get an `io::ErrorKind` value.
	// The enum `io::ErrorKind` is provided by the standard library and has variants representing
	// the different kinds of errors that might result from an `io` operation.

	// The condition we want to check in the inner match is whether the value returned by `error.kind()`
	// is the `NotFound` variant of the `ErrorKind` enum.
	// If it is, we try to create the file with `File::create`.
	// However, because `File::create` could also fail, we need a second arm in the inner `match` expression.
	// When the file can't be created, a different error message is printed.

	// The `Result<T, E> type has many methods that accept a closure and are implemented using `match` expressions.
	// Using those methods will make your code more concise.
	// A more seasoned Rustacean might write this code instead.
	let f = File::open("hello.txt"). unwrap_or_else(|error| {
		if error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|error| {
				panic!("Problem creating the file: {:?}", error);
			})
		} else {
			panic!("Problem opening the file: {:?}", error);
		}
	});
	// Although this code has the same behavior, it doesn't contain any `match` expressions and is cleaner to read.
}

fn shortcuts() {
	// The `Result<T, E>` type has many helper methods defined on it to do various tasks.
	// One of those methods, called `unwrap` is a shortcut method implemented
	// just like the `match` expression we wrote.
	// If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`.
	// If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us.
	use std::fs::File;

	let f = File::open("hello.txt").unwrap();
	// If we run this code without a hello.txt file, we'll see an error message
	// from the `panic!` call that the `unwrap` method makes.

	// Another method, `expect`, which is similar to `unwrap`,
	// lets us also choose the `panic!` error message.
	// Using `expect` instead of `unwrap` and providing good error messages can convey your
	// intent and make tracking down the source of a panic easier.
	let f = File::open("hello.txt").expect("Failed to open hello.txt");
	// We use `expect` in the same way as `unwrap`: to return the file handle or call the `panic!` macro.
	// The error message used by `expect` in its call to `panic!` will be the parameter
	// that we pass to `expect`, rather than the default `panic!` message that `unwrap` uses.

	// If we use `unwrap` in multiple places, it can take more time to figure out exactly which
	// `unwrap` is causing the panic because all `unwrap` calls that panic print the same message.
}

fn propagating_errors() {
	// When you're writing a function whose implementation calls something that might fail,
	// instead of handling the error within this function,
	// you can return the error to the calling code so that it can decide what to do.
	// This is known as propagating the error and gives more control to the calling code,
	// where there might be more information or logic that dictates how the error should be hanndled
	// than what you have available in the context of your code.

	// This function reads a username from a file.
	// If the file doesn't exist or can't be read, this function will return those
	// errors to the code that called this function.
	use std::fs::File;
	use std::io;
	use std::io::Read;

	fn read_username_from_file() -> Result<String, io::Error> {
		let f = File::open("hello.txt");

		let mut f = match f {
			Ok(file) => file,
			Err(e) => return Err(e),
		};

		let mut s = String::new();

		match f.read_to_string(&mut s) {
			Ok(_) => Ok(s),
			Err(e) => Err(e),
		}
	}
	// The return type is `Result<String, io::Error>`.
	// This means the function is returning a value of the type `Result<T, E>`
	// where the generic parameter `T` has been filled in with the concrete type `String`
	// and the generic type `E` has been filled in with the concrete type `io::Error`.
	
	// If this function succeeds without any problems, the code that calls this function
	// will receive an `Ok` value that holds a `String` - the username that this function read from the file.
	// If this function encounters any problems, the code that calls this function will receive
	// an `Err` value that holds an instance of `io::Error` that contains more information
	// about what the problems were.
	// We chose `io::Error` as the return type of this function because that happens to be
	// the type of the error value returned from both of the operations we're calling in this
	// function's body that might fail: the `File::open` function and the `read_to_string` method.

	// If the calling code gets an `Err` value, it could call `panic!` and crash the program,
	// use a default username, or look up the username from somewhere other than a file, for example.
	// We don't have enough information on what the calling code is actually trying to do,
	// so we propagate all the success or error information upward for it to handle appropriately.
}

fn error_operator() {
	// The following implementation has the same functionality as the function above,
	// but it uses the `?` operator.
	use std::fs::File;
	use std::io;
	use std::io::Read;

	fn read_username_from_file() -> Result<String, io::Error> {
		let mut f = File::open("hello.txt");
		let mut s = String::new();
		f.read_to_string(&mut s)?;
		Ok(s)
	}
	// The `?` placed after a `Result` value is defined to work in almost the same way
	// as the `match` expressions we defined to handle the `Result` values previously.
	// If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned
	// from this expression, and the program will continue.
	// If the value is an `Err`, the `Err` will be returned from the whole function as if
	// we had used the `return` keyword so the error value gets propagated to the calling code.

	// There is a difference between what the `match` expression does and what the `?` operator does:
	// error values that have the `?` operator called on them go through the `from` function,
	// defined in the `From` trait in the standard library, which is used to convert errors
	// from one type into another.
	
	// When the `?` operator calls the `from` function, the error type received is converted into
	// the error type defined in the return type of the current function.
	// This is useful when a function returns one error type to represent all the ways a function might fail,
	// even if parts might fail for many different reasons.
	// As long as each error type implements the `from` function to define how to convert itself
	// to the returned error type, the `?` operator takes care of the conversion automatically.

	// We could even shorten this code further by chaining method calls immediately after the `?`:
	fn read_username_from_file_2() -> Result<String, io::Error> {
		let mut s = String::new();

		File::open("hello.txt")?.read_to_string(&mut s)?;

		Ok(s)
	}


	// There;s a way to make this even shorter:
	fn read_username_from_file_3() -> Result<String, io::Error> {
		std::fs::read_to_string("hello.txt")
	}
	// Reading a file into a string is a fairly common operation, so Rust provides the convenient
	// `fs::read_to_string` function that opens the file, creates a new `String`,
	// reads the contents of the file, puts the contents into that `String`, and returns it.
}

// The `?` operator is only allowed in a function that returns `Result` or `Option` or another type
// that implements `std::ops::Try`.

// The main function is special, and there are restrictions on what its return type must be.
// One valid return type for main is `()`, and conveniently, another valid return type is `Result<T, E>`.
use std::error::Error;
use std::fs::File;

fn is_main() -> Result<(), Box<dyn Error>> {
	let f = File::open("hello.txt")?;

	Ok(())
}
// The `Box<dyn Error>` type is called a trait object.
// For now, you can read `Box<dyn Error>` to mean "any kind of error".
// Using `?` in a `main` function with this return type is allowed.