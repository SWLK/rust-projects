#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.height && self.height > other.height
    }

    // Associated functions are often used for constructors
    // that will return a new instance of the struct.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let square1 = Rectangle::square(3);

    println!(
        "The new square is: {:#?}",
        square1
    );
}
