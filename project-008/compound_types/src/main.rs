fn main() {
    let tup: (i32, f32, i8) = (100, 3.14, 1);
    let (x, y, z) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("tup = ({}, {}, {})", tup.0, tup.1, tup.2);
}
