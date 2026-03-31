#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(val: u32) -> Rectangle {
        Rectangle {
            width: val,
            height: val
        }
    }
}

fn main() {
    println!("Struct methods and associated functions!");
    let rect1 = Rectangle::square(20);
    let rect2 = Rectangle { width: 10, height: 10 };
    println!("Rect1 : {:#?} area : {}", rect1, rect1.area());
    println!("Rect2 : {:#?} area : {}", rect2, rect2.area());
    println!("rect1 can hold rect2 ? {}", rect1.can_hold(&rect2));
}
