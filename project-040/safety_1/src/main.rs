fn main() {
    println!("Demo of safety!");
    let _ptr: &String;
    {
        let s = String::from("Hello");
        _ptr = &s;
    }
    // Compilation erorr: 's' does not live long enough if the below line uncommented
    // println!("{}", ptr);
}

/* Equivalent C++ code
#include <iostream>
#include <vector>
#include <string>

int main() {
    std::string* ptr;
    {
        std::string s = "Hello";
        ptr = &s;
    } // s is destroyed here
    // Undefined behavior: ptr is dangling
    std::cout << *ptr << std::endl;
}
*/
