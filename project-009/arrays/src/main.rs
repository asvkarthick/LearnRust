fn main() {
    println!("Array demp!");
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [100; 5]; // 5 elements of value 100
    let arr3: [i32; 5] = [10, 20, 30, 40, 50];
    println!("arr1 = [{}, {}, {}, {}, {}]", arr1[0], arr1[1], arr1[2], arr1[3], arr1[4]);
    println!("arr2 = [{}, {}, {}, {}, {}]", arr2[0], arr2[1], arr2[2], arr2[3], arr2[4]);
    println!("arr3 = [{}, {}, {}, {}, {}]", arr3[0], arr3[1], arr3[2], arr3[3], arr3[4]);
}
