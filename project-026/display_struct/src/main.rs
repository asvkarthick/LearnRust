#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32
}

fn main() {
    println!("Hello, world!");
    let rect = Rectangle {
        width: 20,
        height: 5
    };
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);
}
