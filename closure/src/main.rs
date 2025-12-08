// fn using_closure_as_parameter_and_capture_environment() {
    
//     fn add(x: i32, y:i32) -> i32 {
//         x + y
//     }

//     // story here changes dramatically.
//     // Fn is a trait, which is needed to be dispatched at the runtime
//     // Box puts that function into heap
//     fn calculator(operation: Box<dyn Fn() -> i32 + '_>) {
//         let result = operation();
//         println!("result of operation {}", result);    
//     }

//     //calculator(Box::new(add)); 
//     //calculator(1,2,Box::new(|x,y| x + y)); // works as expected

//     let z = 3;
//     let x = 2;
//     let y = 5;
//     calculator(1,2,Box::new(|| x + y + z)); 
//     // z is an unexpected guess in our closure, because it's not passed,
//     // between pipes, but due to the nature of closures which captures the environment I can stil access it and need to make sure to incdicate it's lifetime.

// }

// fn capture_modify_environment() {
//     let mut result = 0;

    
    
//     // Using FnMut trait
//     let mut calculator: Box<dyn FnMut(i32, i32)> = Box::new(|x, y| { result = x + y });
//     calculator(1, 2);
//     drop(calculator);
//     println!("{}", result);  // Output: 3
// }

fn capture_ownership_modify() {
    let nums = vec![1, 2, 3, 4, 5].into_iter();
    //let product_through_iterator: Box<dyn FnOnce() -> i32> = Box::new(move || nums.product());
    //let product_through_iterator  = Box::new(move || nums.product());
    let result: i32 = product_through_iterator();
    println!("{}", result);  // Output: 120
}


fn main()
{
    //using_closure_as_parameter_and_capture_environment();
    capture_modify_environment();
}