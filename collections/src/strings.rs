pub mod strings {
	pub fn main() {
		// Rust has only one string type in the core language,
		// which is the string slice `str` that is usually
		// seen in its borrowed form `&str`.

		// The `String` type, which is provided by Rust's standard library
		// rather than coded into the core language,
		// is a growable, mutable, owned, UTF-8 encoded string type.

		// Many of the same operations available with `Vec<T>`
		// are available with `String` as well,
		// such as the `new` function to create a string.
		let mut s = String::new();

		// Often, we'll have some initial data that we want to start the string with.
		// For that, we use the `to_string` method, which is available on any type
		// that implements the `Display` trait, as string literals do.
		let data = "initial contents";

		let s = data.to_string();

		// the method also works on a literal directly:
		let s = "inital contents".to_string();

		// We can also use the function `String::from`
		// to create a `String` from a string literal.
		let s = String::from("initial contents");

		// Strings are UTF-8 encoded, so we can include any
		// properly encoded data in them.
		let hello = String::from("你好");
	}

	pub fn updating() {
		// A `String` can grow in size and its contents can change,
		// just like the contents of a `Vec<T>`.

		// We can grow a `String` by using the `push_str` method
		// to append a string slice:
		let mut s = String::from("foo");
		s.push_str("bar");
		// The `push_str` method takes a string slice because
		// we don't necessarily want to take ownership of the parameter.
		let mut s1 = String::from("foo");
		let s2 = "bar";
		sl.push_str(s2);
		// println!("s2 is {}", s2);    // error
		// It avoids situations like this where we would not be able
		// to use `s2` after appending its contents to `s1`.

		// The `push` method can take a single character as a parameter.
		let mut s = String::from("lo");
		s.push('l');

		// You can use the `+` operator or the `format!` macro
		// to concatenate `String` values.
		let s1 = String::from("Hello, ");
		let s2 = String::from("world!");
		let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
		// The reason `s1` is no longer valid after the addition and
		// the reason we used a reference to `s2` has to do with the 
		// signature of the mthod that gets called when we use the `+` operator.
		// The `+` operator uses the `add` method, whose signature looks like this:
		// fn add(self, s: &str) -> String {}

		// `s2` has an `&`, meaning we're adding a reference of the
		// second string to the first string.
		// We can only add a `&str` to a `String`;
		// we cant add two `String` values together.

		// The type of `&s2` is `&String`, not `&str`,
		// as specified in the second parameter to add.
		// The reason it compiles is because the compiler can "coerce"
		// the `&String` argument into a `&str`.
		// When we call the `add` method, Rust uses a "deref coercion",
		// which here turns `&s2` into `&s2[..]`.

		// Although `let s3 = s1 + &s2;` looks like it will copy both strings
		// and create a new one, this statement actually takes ownership of `s1`,
		// appends a copy of the cotents of `s2`, and then returns ownership
		// of the result.

		// If we need to concatenate multiple strings,
		// the behavior of the `+` operator gets unwieldy.
		// For more complicated string combining,
		// we can use the `format!` macro:
		let s1 = String::from("tic");
		let s2 = String::from("tac");
		let s3 = String::from("toe");

		let s = format!("{}-{}-{}", s1, s2, s3);
		// The `format!` macro works in the same way as `println!`,
		// but instead of printing the output to the screen,
		// it returns a `String` with the contents.
		// It also doesn't take ownership of any of its parameters.
	}

	pub fn indexing() {
		// If you try to access parts of a `String` using
		// indexing syntax in Rust, you'll get an error.
		let s1 = String::from("hello");
		// let h = s1[0];	// error

		// A `String` is a wrapper over a `Vec<u8>`.
		// Cyrillic letters for example, each Unicode scalar value
		// takes 2 bytes of storage.
		// Therefore, an index into the string's bytes will not always
		// correlate to a valid Unicode scalar value.

		// If you need to perform operations on individual Unicode scalar values,
		// the best way to do so is to use the `chars` method.
		for c in "नमस्ते".chars() {
			println!("{}", c);
		}
		// This will return six values of type `char`.

		// The `bytes` method returns each raw byte.
		for b in "नमस्ते".bytes() {
			println!("{}", b);
		}
		// This code will print the 18 bytes that make up the `String`.
	}
}