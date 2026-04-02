use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    println!("BinaryHeap Example!");
    let mut heap = BinaryHeap::new();

    // Adding elements
    heap.push(30);
    heap.push(10);
    heap.push(20);
    heap.push(40);

    // Peeking (largest) element
    println!("max: {:?}", heap.peek());

    // Removing elements
    println!("popped: {:?}", heap.pop());
    println!("popped: {:?}", heap.pop());

    heap.push(5);
    heap.push(15);
    let sorted_desc: Vec<i32> = heap.into_sorted_vec();
    println!("sorted: {sorted_desc:?}");

    // Min Heap
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(30));
    min_heap.push(Reverse(10));
    min_heap.push(Reverse(50));

    println!("min: {:?}", min_heap.peek());
    println!("popped: {:?}", min_heap.pop());

    let heap2 = BinaryHeap::from(vec![4, 1, 7, 3]);
    for val in heap2 { // iteration order is NOT guaranteed to be sorted
        print!("{val} ");
    }
    println!();
}
