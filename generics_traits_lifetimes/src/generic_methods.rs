// We can implement methods on structs and enums and use generic types in their definitions, too.
struct Point<T> {
    x: T,
    y: T,
}

// We have to declare `T` just after `impl` so we can use it to specify that we're implementing
// methods on the type `Point<T>`.
// By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle
// brackets in `Point` is a generic type rather than a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We could implement methods only on `Point<f32>` instances rather than on `Point<T>` instances
// with any generic type.
// This code means the type `Point<f32>` will have a method named `distance_from_origin` and other
// instances of `Point<T>` where `T` is not of type `f32` will not have this method defined.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ===================================================================

struct Different_Point<T,U> {
    x: T,
    y: U,
}

// Generic type parameters in a struct definition aren't always the same as those you use in that
// struct's method signatures.
// The method takes another `Different_Point` as a parameter, which might have different types from
// the `self` `Different_Point` we're calling `mixup` on.
// The method creates a new `Point` instance with the `x` value from the `self` `Different_Point`
// (of type `T`) and the `y` value from the passed-in `Point` (of type `W`).
impl<T,U> Different_Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
    // Here, the generic parameters `T` and `U` are declared after `impl`, because they go with the
    // struct definition.
    // The generic parameters `V` and `W` are declared after `fn mixup`, because they're only
    // relevant to the method.
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = Different_Point { x:5, y: 10.4 };
    let p2 = Different_Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Rust implements generics in such a way that your code doesn't run any slower using generic types
// than it would with concrete types.
// Rust accomplishes this by performing monomorphization of the code that is using generics at
// compile time.
// Monomorphization is the process of turning generic code into specific code by filling in the
// concrete types that are used when compiled.
