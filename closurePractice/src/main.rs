// fn main() {
//     let operation = |a: i32, b: i32| {
//         // Your implementation here
//         a * b
//     };

//     println!("Result: {}", operation(10, 5));
// }


fn main() {
    let mut total = 0;
    let mut accumulate = || {
        total += 5;
        println!("Total: {}", total);
    };
    accumulate(); // Output: Total: 5
    accumulate(); // Output: Total: 10

}