fn main() {
    println!("Demo of string slice parameters!");
    let s1 = String::from("String slice parameters");
    let s2 = "String slice parameters";

    println!("s1 = {}, first word = {}", s1, first_word(&s1));
    println!("s2 = {}, first word = {}", s2, first_word(&s2[..]));
    println!("s2 = {}, first word = {}", s2, first_word(s2));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
