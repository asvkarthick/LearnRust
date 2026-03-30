fn main() {
    let mut s= String::from("Ownership");
    s.push_str(" demo!");
    println!("{s}");

    let s1 = String::from("Hi");
    // let s2 = s1;
    println!("{s1}");

    let mut str1 = String::from("Hi");
    str1 = String::from("Hello");
    println!("str1 : {str1}");

    let str2 = String::from("String");
    let str3 = str2.clone();
    println!("str2 : {str2}, str3: {str3}");

    let num1 = 10;
    let num2 = num1;
    println!("num1 : {num1}, num2 : {num2}");

    let str = String::from("string");
    takes_ownership(str);
    // println!("str : {str}");

    let i = 20;
    makes_copy(i);
    println!("i : {i}");
}

fn takes_ownership(str : String) {
    println!("str in takes_ownership fn : {str}");
}

fn makes_copy(i : i32) {
    println!("int : {i}");
}
