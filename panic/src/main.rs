fn read_username_from_file() -> Result<String, io::Error> {
    ok("hello".to_string())
}

use std::io::ErrorKind;
fn main()
{
    le r = read_username_from_file_3ver();
    println!("{:?}",r)
}




// fn main() {
//     //println!("Panic");
//     panic_examples();
// }

// fn panic_examples() {
//     println!("Everything is good");

    
//     // panic!("Crash the program, stop running, clean the memory");
//     // println!("This won't be printed.");
//     let v = vec![1, 2, 3];
//     println!("{:?}", v[99]); // This will cause a panic because the index is out of bounds
// }