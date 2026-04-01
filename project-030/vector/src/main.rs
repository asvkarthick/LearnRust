fn main() {
    println!("Another vector example!");
    let v1 = vec![1, 2, 3, 4, 5];
    println!("Contents of vec v1");
    for i in v1 {
        println!("{}", i);
    }
    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);
    println!("Contents of vec v2");
    for i in v2 {
        println!("{}", i);
    }
}
