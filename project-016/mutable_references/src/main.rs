fn main() {
    println!("Demo of mutable references!");
    let mut s1 = String::from("Mutable ");
    change(&mut s1);
    println!("s1 = {}", s1);
}

fn change(str: &mut String) {
    str.push_str("references");
}
