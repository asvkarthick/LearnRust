use std::collections::LinkedList;
fn main() {
    println!("Linked list example!");
    let mut list = LinkedList::new();

    // Adding elements
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_front(5);

    // Accessing elements
    println!("Front: {:?}", list.front());
    println!("Back : {:?}", list.back());

    // Removing elements
    list.pop_front();
    list.pop_back();

    // Merging two lists
    let mut list2 = LinkedList::new();
    list2.push_back(100);
    list2.push_back(200);
    list.append(&mut list2); // moves all elements from list2 to list
    
    println!("Values of list");
    for val in &list {
        print!("{val} ");
    }
    println!();

    println!("len: {}", list.len());
    list.clear();
}
