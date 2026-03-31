fn main() {
    println!("Demo of ownership and copy!");
    let s = String::from("Hello");
    takes_ownership(s);
    // we cannot use s here after because we transferred ownership to the function
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(str: String) {
    println!("String = {}", str);
}

fn makes_copy(num: i32) {
    println!("Number = {}", num);
}
