struct Rectangle {
    width:  u32,
    height: u32
}

fn main() {
    println!("Rectangle with struct!");
    let rect = Rectangle {
        width: 20,
        height: 5
    };
    println!("Rectangle: width: {}, height: {}, area: {}", rect.width, rect.height, compute_area(&rect));
}

fn compute_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
