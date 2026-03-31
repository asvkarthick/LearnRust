struct Student {
    name: String,
    age : u32,
    id  : u32
}

fn main() {
    println!("Demo of mutable struct!");
    let s1 = build_student(String::from("KK"), 20, 1);
    println!("Student name : {}, age : {}, id : {}", s1.name, s1.age, s1.id);
}

fn build_student(name: String, age: u32, id: u32) -> Student {
    Student {
        name: name,
        age : age,
        id  : id
    }
}
