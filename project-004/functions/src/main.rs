fn main() {
    println!("Hello, world!");
    second_function();
    func_with_argument(5);
    println!("Function return value : {}", func_with_return_value());
    println!("Func with arg ret val : {}", func_with_arg_ret_val(20));
}

fn second_function() {
    println!("Executing second function!");
    let x = {
        let y = 20;
        y + 1
    };
    println!("x : {x}");
}

fn func_with_argument(x : i32) {
    println!("Argument value x : {x}");
}

fn func_with_return_value() -> i32 {
    5
}

fn func_with_arg_ret_val(x : i32) -> i32 {
    x * 2
}
