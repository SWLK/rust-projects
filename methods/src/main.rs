#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // The method parameter could be an immutable borrow of self (&self),
    // a mutable borrow of self (&mut self),
    // or taking ownership (self), thought that is rare and would only be used
    // if you want to change self into something else and invalidate the original instance.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
