fn main() {
    println!("Demo of string slice type");
    let s = String::from("String Slice");
    let s1 = &s[0..6];
    let s2 = &s[7..];

    println!("s = {}", s);
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
