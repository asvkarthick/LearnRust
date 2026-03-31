struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn compute_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Struct methods example!");
    let rect = Rectangle {
        width: 20,
        height: 10
    };
    println!("Rectangle width: {}, height: {}, area: {}", rect.width, rect.height, rect.compute_area());
}
