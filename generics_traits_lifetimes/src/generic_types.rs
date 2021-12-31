fn main() {
	// We can use generics to create definitions for items like function signatures or structs,
    // which we can then use with many different concrete data types.
    
    // When defining a function that uses generics, we place the generics in the
    // signature of the function where we would usually specify the data types
    // of the parameters and return value.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    // The `largest_i32` function finds the largest `i32` in a slice.
    // The `largest_char` function finds the largest `char` in a slice.
    // The function bodies have the same code, so we can eliminate
    // the duplication by introducing a generic type parameter in a single function.
}

// To parameterize the types in the new function we'll define,
// we need to name the type parameter, just as we do for the value parameters to a function.
// You can use any identifier as a type parameter name.
// We'll use `T` because, by convention, parameter names in Rust are short, often just a letter,
// and Rust's type-naming convention is CamelCase.
//
// When we use a parameter in the body of the function, we have to declare the parameter name in
// the signature so the compiler knows what that name means.
// Similarly, when we use a type parameter name in a function signature, we haveto declare the type
// parameter name before using it.
// To define the generic `largest` function, place type name declarations inside angle brackets,
// `<>`, between the name of the function and the parameter list:
fn largest<T>(list: &[T]) -> T {
    // We read this definition as: the function `largest` is generic over some type `T`.
    // The function has one parameter named `list`, which is a slice of values of type `T`.
    // The `largest` function will return a value of the same type `T`.
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
    // If we compile this function, we will get an error.
    // The error states that the body of `largest` won't work for all possible types that `T` could
    // be. Because we want to compare values of type `T` in the body, we can only use types whose
    // values can be ordered.
    // To enable comparisons, the standard library has the `std::cmp::PartialOrd` trait that you
    // can implement on types.
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
