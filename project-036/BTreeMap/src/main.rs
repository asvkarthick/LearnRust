use std::collections::BTreeMap;

fn main() {
    println!("BTreeMap example!");
    let mut map = BTreeMap::new();

    // Adding elements
    map.insert(3, "three");
    map.insert(2, "two");
    map.insert(1, "one");
    map.insert(4, "four");

    // Entry API
    map.entry(5).or_insert("five");

    // Accessing elements
    println!("key 2: {:?}", map.get(&2));

    // Removing elements
    map.remove(&4);

    for (k, v) in &map {
        println!("{k} : {v}");
    }

    // Range queries
    for (k, v) in map.range(2..=3) {
        println!("range: {k} -> {v}");
    }

    println!("First key value : {:?}", map.first_key_value());
    println!("Last key value : {:?}", map.last_key_value());
}
