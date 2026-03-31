fn main() {
    println!("Demo of string slice!");
    let s1 = String::from("Slice type");
    let len = first_word(&s1);
    println!("s1 = {}, first word len = {}", s1, len);
}

fn first_word(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}
