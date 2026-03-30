fn main() {
    println!("Demonstration of loops");

    let mut counter = 0;
    loop {
        println!("Counter = {}", counter);
        if counter == 5 {
            println!("Exiting loop!");
            break;
        }
        counter = counter + 1;
    }

    counter = 0;
    while counter < 5 {
        println!("While loop counter = {}", counter);
        counter = counter + 1;
    }

    for i in 0..5 {
        println!("for loop counter = {}", i);
    }
}
