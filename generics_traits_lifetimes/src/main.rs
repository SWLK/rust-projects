// Generics are abstract stand-ins for concrete types or other properties.
// Similar to the way a function takes parameters with unknown values to run the same code
// on multiple concrete values, functions can take parameters of some generic type instead of a concrete type.
pub fn main() {
	// Let's look at how to remove duplication that doesn't involve generic types by extracting a function.
	// In the same way that you recognize duplicated code to extract into a function,
	// you'll start to recognize duplicated code that can use generics.
	let number_list = vec![34, 50, 25, 100, 65];

	let mut largest = number_list[0];

	for number in number_list {
		if number > largest {
			largest = number;
		}
	}

	println!("The largest number is {}", largest);
	
	// To find the largest number in two different lists of numbers,
	// we can duplicate the code and use the same logic at two different places in the program.
	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

	let mut largest = number_list[0];

	for number in number_list {
		if number > largest {
			largest = number;
		}
	}

	println!("The largest number is {}", largest);
	// Although this code works, duplicating code is tedious and error prone.
	// We also have to update the code in multiple places when we want to change it.

	// To eliminate this duplication, we can create an abstraction by defining a function
	// that operates on any list of integers given to it in a parameter.
	// This solution makes our code clearer and lets us express the concept of finding
	// the largest number in a list abstractly.
	let number_list = vec![34, 50, 25, 100, 65];

	let result = largest_func(&number_list);
	println!("The largest number is {}", result);

	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

	let result = largest_func(&number_list);
	println!("The largest number is {}", result);
	// In sum, here are the steps we took to change the code:
	// 1. Identify duplicate code.
	// 2. Extract the duplicate code into the body of the function and
	//    specify the inputs and return values of that code in the function signature.
	// 3. Update the two instances of duplicated code to call the funciton instead.
}


fn largest_func(list: &[i32]) -> i32 {
	let mut largest = list[0];

	// We aren't referencing a reference to an `i32` here;
	// we're pattern matching and destructuring each `i32` that the for loop gets
	// so that `item` will be an `i32` inside the loop body.
	for &item in list {
		if item > largest {
			largest = item
		}
	}

	largest
}
