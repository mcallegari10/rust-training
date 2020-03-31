#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rec: &Rectangle) -> bool {
        self.area() >= another_rec.area()
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };

    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("rect is: {:#?}", rect);
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect2 hold rect? {}", rect2.can_hold(&rect));

    let req = Rectangle::square(25);
}
