#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width: 10, height: 50};
    let square1 = Rectangle::square(5);
    // Use debug trait to print out the struct.
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
