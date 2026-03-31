fn main() {
    println!("Demo of references!");
    let s1 = String::from("References");
    let len = compute_length(&s1);
    println!("s1 = {}, len = {}", s1, len);
}

fn compute_length(str: &String) -> usize {
    str.len()
}
