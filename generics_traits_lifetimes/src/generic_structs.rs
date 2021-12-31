// We can also define structs to use a generic type parameter in one more more fields using the
// `<>` syntax.
struct Point<T> {
// The syntax for using generics in struct definitions is similar to that used in function
// definitions. First, we declare the name of the type parameter inside angle brackets just after
// the name of the struct. Then we can use the generic type in the struct definition where we would
// otherwise specify concrete data types.
    x: T,
    y: T,
}

struct Different_Type_Points<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point{ x: 5, y: 10 };
    let float = Point{ x: 1.0, y: 4.0 };

    let integer_and_float = Different_Type_Points{ x: 5, y: 4.0 };
}
