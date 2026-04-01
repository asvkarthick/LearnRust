fn main() {
    println!("Vector example!");
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3, 4, 5];
    v.push(10);
    v.push(20);
    v.push(30);

    for (i, item) in v.into_iter().enumerate() {
        println!("v[{}] = {}", i, item);
    }
    for (i, item) in v1.into_iter().enumerate() {
        println!("v1[{}] = {}", i, item);
    }
}
