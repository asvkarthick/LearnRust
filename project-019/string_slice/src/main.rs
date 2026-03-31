fn main() {
    println!("Demo of String slice!");
    let s = String::from("Hello World");
    let s1 = first_word(&s);
    println!("s = {}", s);
    println!("s1 = {}", s1);
}

fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}
