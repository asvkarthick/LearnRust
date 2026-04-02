fn main() {
    println!("Vector example!");
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3, 4];

    // Adding elements
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.insert(1, 15);
    v.extend([50, 60]);

    // Accessing elements
    println!("First: {}", v[0]);
    println!("Safe:  {:?}", v.get(100));

    // Removing elements
    v.pop();
    v.remove(1);
    v.retain(|&x| x > 15); // keep only elements > 15
    
    // Iterators
    for val in &v {
        print!("{val} ");
    }
    println!();

    // Mutable iteration
    for val in &mut v {
        *val *= 2;
    }

    // functional style
    let sum: i32 = v.iter().sum();
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

    println!("len: {}, is_empty: {}", v.len(), v.is_empty());
    println!("contains 40: {}", v.contains(&40));
    v.sort();
    v.dedup(); // remove consecutive duplicates
    v.clear();
}
