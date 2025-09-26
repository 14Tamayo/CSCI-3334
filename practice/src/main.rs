fn main() {
    //Problem 1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"


    //Problem 2
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"


    //Problem 3
    // create necessary variables and test your function for low 0 high 100
    // total should be 5050
	let low: i32 = 0;
	let high: i32 = 100;
	let mut total: i32 = 0;
	
	sum(&mut total, low, high);
	
	println!("The total of the sum is {total}");
}

//Problem 1 function
fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
	let mut s3 = s1.clone();
	
	s3.push_str(s2);
	
	s3
}


//Problem 2 function
fn clone_and_modify(s: &String) -> String {
    // Your code here
	let mut s2 = s.clone();
	
	s2.push_str("World");
	
	s2
}



//Problem 3 function
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
	//*total = low + high;		//sum of variables
	*total = (low..=high).sum();        //sum of range
    
}