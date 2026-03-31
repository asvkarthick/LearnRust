fn main() {
    println!("Another demo of ownership!");
    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("Ownership");
    let s3 = takes_and_gives_ownership(s2);
    println!("s3 = {}", s3);
}

fn gives_ownership() -> String {
    let str = String::from("gives_ownership");
    str
}

fn takes_and_gives_ownership(str: String) -> String {
    str
}
