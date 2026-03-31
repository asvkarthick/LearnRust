struct Student {
    name: String,
    age : u32,
    id  : u32
}

fn main() {
    println!("Demo of struct!");

    let s1 = Student {
        name: String::from("KK"),
        age : 20,
        id  : 1
    };

    println!("Student name : {}, age : {}, id : {}", s1.name, s1.age, s1.id);
}
