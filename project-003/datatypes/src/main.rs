fn main() {
    println!("Data types!");

    let x : u32 = 100;
    let y : u8 = 129;
    let z : i8 = 100;
    let n : u32 = "42".parse().expect("Not a number!");

    println!("x : {x}");
    println!("y : {y}");
    println!("z : {z}");
    println!("n : {n}");

    let hex : u32 = 0x1234;
    println!("hex : {hex}");
    let oct : u32 = 0o123;
    println!("oct : {oct}");
    let byte : u8 = b'A';
    println!("byte : {byte}");

    let ch : char = 'A';
    println!("ch : {ch}");

    let t : bool = true;
    println!("t : {t}");

    let sum = 5 + 10;
    let diff = 10 - 5;
    let mul = 10 * 5;
    let div = 10 / 5;
    let rem = 12 % 5;
    println!("sum : {sum}");
    println!("diff : {diff}");
    println!("mul : {mul}");
    println!("div : {div}");
    println!("rem : {rem}");
}
