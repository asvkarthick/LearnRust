fn main() {
    println!("Safety example!");
    let mut arr = [1, 2, 3, 4, 5];
    for item in &mut arr {
        *item = 0;
    }

    match arr.get(10) {
        Some(val) => println!("Value: {val}"),
        None      => println!("Index out of bounds!"),
    }
}

/*
#include <stdio.h>

int main() {
    int arr[5] = {1, 2, 3, 4, 5};

    // Undefined Behavior: writing past the end of the array
    for (int i = 0; i < 10; i++) {
        arr[i] = 0; // Corrupts stack memory silently
    }
    return 0;
}
*/
