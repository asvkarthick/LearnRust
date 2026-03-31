struct Student {
    name: String,
    age : u32,
    id  : u32
}

fn main() {
    println!("Mutable struct type!");
    let mut s1 = Student {
        name: String::from("KK"),
        age : 20,
        id  : 1
    };
    s1.name = String::from("ASVKK");
    println!("Student name : {}, age : {}, id : {}", s1.name, s1.age, s1.id);
}
