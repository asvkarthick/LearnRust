use std::collections::HashMap;

fn find_user(users: &HashMap<i32, String>, id: i32) -> Option<&String> {
    users.get(&id) // Return Option<&String>
}

fn main() {
    println!("Safety example 3!");
    let mut users = HashMap::new();
    users.insert(1, "Alice".to_string());

    match find_user(&users, 99) {
        Some(name) => println!("Found: {name}"),
        None       => println!("User not found!"),
    }

    let name = find_user(&users, 99).unwrap_or(&"Unknown".to_string());
    // println!("User: {name}");
}

/*
#include <iostream>
#include <map>
#include <string>

std::string* find_user(std::map<int, std::string>& users, int id) {
    auto it = users.find(id);
    if (it != users.end()) return it->second;
    return nullptr;
}

int main() {
    std::map<int, std::string> users;
    users[1] = "Aadhi";
    std::string* user = find_user(users, 99);
    // SEGFAULT: dereferencing null pointer!
    std::cout << *user << std::endl;
}
*/
