use std::collections::HashSet;

fn main() {
    println!("Hash Set example!");
    let mut set = HashSet::new();

    // Adding elements
    set.insert(10);
    set.insert(20);
    set.insert(30);
    set.insert(10);

    // Checking
    println!("Checking 20? {}", set.contains(&20));

    // Removing
    set.remove(&30);
    set.retain(|&x| x > 5);

    // Iterating elements
    for val in &set {
        print!("{val} ");
    }
    println!();

    // Set operations
    let a: HashSet<i32> = [1, 2, 3, 4].into();
    let b: HashSet<i32> = [3, 4, 5, 6].into();

    let union: HashSet<_> = a.union(&b).collect();
    let intersection: HashSet<_> = a.intersection(&b).collect();
    let difference: HashSet<_> = a.difference(&b).collect();
    let sym_diff: HashSet<_> = a.symmetric_difference(&b).collect();

    println!("union:        {union:?}");
    println!("intersection: {intersection:?}");
    println!("difference:   {difference:?}");
    println!("sym_diff:     {sym_diff:?}");

    // Subset / Superset
    println!("a is subset of b? {}", a.is_subset(&b));
}
