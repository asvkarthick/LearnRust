use std::collections::BTreeSet;

fn main() {
    println!("BTreeSet example - Sorted Set!");
    let mut set = BTreeSet::new();

    // Adding elements
    set.insert(30);
    set.insert(20);
    set.insert(10);
    set.insert(50);
    set.insert(40);

    // Iterating elements
    for val in &set {
        print!("{val} ");
    }
    println!();

    // Removing elements
    set.remove(&30);

    // Range queries
    for val in set.range(15..=40) {
        print!("{val} ");
    }
    println!();

    println!("First: {:?}", set.first());
    println!("last:  {:?}", set.last());

    let set2: BTreeSet<i32> = [20, 60, 80].into();
    let union: BTreeSet<_> = set.union(&set2).collect();
    println!("union: {union:?}");

    set.pop_first();
    set.pop_last();
}
