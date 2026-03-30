fn main() {
    println!("Control flow demonstration");

    let num = 5;
    if num < 10 {
        println!("Number is less than 10");
    } else {
        println!("Number is greater than 9!");
    }

    let x = -1;
    if x < 0 {
        println!("x is less than 0!");
    } else if x > 0 {
        println!("x is greater than 0!");
    } else {
        println!("x is equal to 0!");
    }

    let x : i32 = 25;
    if x < 0 {
        println!("{x} is less than 0");
    } else {
        println!("{x} is greater than or equal to 0!");
    }

    let condition = true;
    if condition {
        println!("condition is true!");
    } else {
        println!("condition is false!");
    }

    let val = if condition {10} else {20};
    println!("val : {val}");

    let mut iter = 0;
    loop {
        println!("Looping...");
        iter += 1;
        if iter == 100 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("number : {number}");
        number -= 1;
    }
    println!("End of while loop");

    let arr = [1, 2, 3, 4, 5];
    for elem in arr {
        println!("elem : {elem}");
    }
    println!("End of array indexing");

    for number in (1..4).rev() {
        println!("{number}");
    }
}
