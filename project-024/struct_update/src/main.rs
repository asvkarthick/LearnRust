struct Student {
    name: String,
    age : u32,
    id  : u32
}

fn main() {
    println!("Struct update syntax!");
    let s1 = Student {
        name: String::from("KK1"),
        age : 20,
        id  : 1
    };
    let s2 = Student {
        name: String::from("KK2"),
        age : s1.age,
        id  : s1.id + 1
    };
    let s3 = Student {
        name: String::from("KK3"),
        id  : s2.id + 1,
        ..s1
    };
    println!("Student-1 name : {}, age : {}, id : {}", s1.name, s1.age, s1.id);
    println!("Student-2 name : {}, age : {}, id : {}", s2.name, s2.age, s2.id);
    println!("Student-3 name : {}, age : {}, id : {}", s3.name, s3.age, s3.id);
}
