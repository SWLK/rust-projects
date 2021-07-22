// Option<T> enum is included in the prelude.
enum Option<T> {
    Some(T),
    None,
}
// `Some` and `None` can be used directly without the `Option::` prefix.
//
// The `<T>` syntax is a generic type parameter.
// That means that the `Some` variant can hold one piece of data of any type.

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // If we use `None` rather than `Some`, we need to tell Rust
    // what type of `Option<T>` we have, because the compiler
    // can't infer the type that the `Some` variant will hold by
    // looking only at the `None` value.
    let absent_number: Option<i32> = None;

    // Since `Option<T>` and `T` are different types,
    // the compiler won't let us use an `Option<T>` value
    // as if it were definitely a valid value.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;     // This won't work or compile
}
