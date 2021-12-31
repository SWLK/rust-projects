// Option<T> is an enum that is generic over type `T` and has two variants:
// `Some`, which holds one value of type `T`, and a
// `None` variant that doesn't hold any value.
// By using the `Option<T>` enum, we can express the abstract concept of having an optional value,
// and because `Option<T>` is generic, we can use this abstraction no matter what the type of the
// optional value is.
enum Option<T> {
    Some(T),
    None,
}

// Enums can use multiple generic types as well.
// The `Result` enum is generic over two types, `T` and `E`, and has two variants:
// `Ok`, which holds a value of type `T`, and
// `Err`, which holds a value of type `E`.
// This definition makes it convenient to use the `Result` enum anywhere we have an operation that
// might succeed (return a value of some type `T`) or fail (return an error of some type `E`).
enum Result<T, E> {
    Ok(T),
    Err(E),
}
