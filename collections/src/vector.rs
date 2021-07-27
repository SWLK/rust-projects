// The collection type `Vec<T>`, also known as vector,
// allows you to store more than one value in a single
// data structure that puts all the values next to
// each other in memory.

// Vectors can only store values of the same type.

pub mod vectors {
	pub fn main() {
		// To create a new, empty vector, we can call the `Vec::new` function.
		let v: Vec<i32> = Vec::new();
		// A type annotation is added because we aren't inserting any values
		// into this vector, therefore Rust doesn't know what type to infer.

		// It's more common to create a `Vec<T>` that has initial values,
		// and Rust provides the `vec!` macro for convenience.
		let v = vec![1, 2, 3];
		// This creates a new `Vec<i32>`, since `i32` is the default integer type.

		// To create a vector and then add elements to it,
		// we can use the `push` method.
		let mut v = Vec::new();

		v.push(5);
		v.push(6);
		v.push(7);
		v.push(8);

		// Like any other `struct`, a vector is freed when it goes out of scope.
		{
			let v = vec![1, 2, 3, 4];

			// do stuff with v
		} // v goes out of scope and is freed here
		// When the vector gets dropped, all of its contents are also dropped,
		// meaning those integers it holds will be cleaned up.
	}

	pub fn reading() {
		// Methods of accessing a value in a vector,
		// either with indexing syntax or the `get` method.
		let v = vec![1, 2, 3, 4, 5];

		// This method uses `&` and `[]`, which gives us a reference.
		let third: &i32 = &v[2];
		println!("The third element is {}", third);

		// This method uses the `get` method, which gives us an `Option<T>`.
		match v.get(2) {
			Some(third) => println!("The third element is {}", third),
			None => println!("There is no third element."),
		};

		// There are two ways to reference an element so you can choose
		// how the program behaves when you try to use an index value
		// that the vector doesn't have an element for.
		// Example 1: &[]
		// This will cause the program to panic because
		// it references a nonexistent element.
		// This method is best used when you want your program to crash
		// if there's an attempt to access an element past the end of the vector.
		let does_not_exist = &v[100];
		// Example 2: get()
		// When the `get` method is passed an index that is outside the vector,
		// it returns `None` without panicking.
		// You would use this method if accessing an element beyond the
		// range of the vector happens occasionally under normal circumstances.
		let does_not_exist = v.get(100);

		// Compiling this code will result in an error:
		let mut v = vec![1, 2, 3, 4, 5];

		let first = &v[0];	// immutable reference

		v.push(6);	// mutable borrow

		println!("The first element is: {}", first);
		// There cannot be mutable and immutable references in the same scope.
		// Adding a new element onto the end of the vector might require
		// allocating new memory and copying the old elements to the new space,
		// if there isn't enough room to put all the elements next to each other
		// where the vector currently is.
		// In that case, the reference to the first element would be pointing to
		// deallocated memory.
	}

	pub fn iterating() {
		// If we want to access each element in a vector in turn,
		// we can iterate through all of the elements rather than
		// use indices to access one at a time.
		let v = vec![100, 32, 57];
		for i in &v {
			println!("{}", i);
		}

		// We can also iterate over mutable references to each element
		// in a mutable vector in order to make changes to all the elements.
		let mut v = vec![100, 32, 57];
		for i in &mut v {
			*i += 50;
		}
		// To change the value that the mutable reference refers to,
		// we have to use the dereference operator (*) to get to
		// the value in `i` before we can use the += operator.
	}

	pub fn multiple_types() {
		// Variants of an enum are defined under the same enum type,
		// so when we need to store elements of a different type in a vector,
		// we can define and use an enum.
		enum SpreadsheetCell {
			Int(i32),
			Float(f64),
			Text(String),
		}

		let row = vec![
			SpreadsheetCell::Int(3),
			SpreadsheetCell::Text(String::from("blue")),
			SpreadsheetCell::Float(10.12),
		];
	}
}