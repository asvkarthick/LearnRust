use std::collections::VecDeque;

fn main() {
    println!("VecDeque example!");
    let mut dq: VecDeque<i32> = VecDeque::new();

    // Adding elements
    dq.push_back(10);
    dq.push_back(20);
    dq.push_back(30);
    dq.push_front(5);
    dq.insert(2, 15);

    // Accessing elements
    println!("Front: {:?}", dq.front());
    println!("Back : {:?}", dq.back());
    println!("Index 2: {}", dq[2]);

    // Removing elements
    dq.pop_front();
    dq.pop_back();
    dq.remove(0);

    dq.extend([1, 2, 3, 4, 5]);
    println!("Value of VecDeque");
    for val in &dq {
        print!("{val} ");
    }
    println!();

    // Convert to vector
    let v: Vec<i32> = dq.into_iter().collect();
    println!("Values of vector");
    for val in &v {
        print!("{val} ");
    }
    println!();
}
