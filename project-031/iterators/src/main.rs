fn main() {
    println!("Iterators example!");
    println!("Sum = {}", sum(10));
}

fn sum(num: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=num {
        sum += i;
    }
    sum
}
