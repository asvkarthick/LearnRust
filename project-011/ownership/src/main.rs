fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // s2 owns "Hello"
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // Uncommenting this will result in compilation error
}
