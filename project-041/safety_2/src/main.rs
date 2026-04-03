fn main() {
    println!("Safety example!");
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2; s1 is no longer valid
    println!("{}", s2);
    // compilation error: value used after move IF UNCOMMENTED
    // println!("{}", s1);
}

/*
#include <stdlib.h>

int main() {
    char* buf = (char*) malloc(100);
    free(buf);
    free(buf);
    return 0;
}
*/
