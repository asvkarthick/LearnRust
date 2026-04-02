use std::collections::HashMap;

fn main() {
    println!("Hash map example!");
    let mut map = HashMap::new();

    // Adding and updating elements
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 5);
    map.insert("four", 4); // overwrite

    // Insert only if key is absent
    map.entry("zero").or_insert(0);

    // Accessing
    println!("one : {:?}", map.get("one"));
    println!("two : {:?}", map.get("two"));
    if let Some(three) = map.get("three") {
        println!("three : {three}");
    }

    map.remove("four");
    map.retain(|_k, v| *v > 0);
    
    println!("len: {}", map.len());
    println!("contains one? {}", map.contains_key("one"));

    for (key, value) in &map {
        println!("{key} : {value}");
    }

    let _keys: Vec<&&str> = map.keys().collect();
    let _values: Vec<&i32> = map.values().collect();

    let _map2: HashMap<&str, i32> = map.into_iter().collect();
}
