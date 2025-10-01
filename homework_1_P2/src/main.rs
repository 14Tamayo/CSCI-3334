fn main() {
    //Array & variable definition
    let intArr = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let mut result: bool;

    //Loop iterates through array calling is_even() on every member of the array
    for element in intArr{
        result = is_even(element);

        if result {
            println!("Number {element} is even");
        }else{
            println!("Number {element} is odd");
        }

        if element % 15 == 0 {
            println!("FizzBuzz");
        }else if element % 3 == 0 {
            println!("Fizz");
        }else if element % 5 == 0 {
            println!("Buzz");
        }

    }

    //Variable definition
    let mut counter = 0;
    let mut sum = 0;

    //Loop iterates through array & adds all the numbers
    while counter < 10 {
        sum += intArr[counter];
        counter += 1;

    }

    println!("The sum of the array values is {sum}");

    //Variable will hold largest number in the array
    let mut largest = intArr[0];

    for element in intArr{
        if element > largest {
            largest = element;
        }
    }

    println!("Largest number in the array is {largest}");

}



//Functions
//is_even returns false if number is not even, returns true if number is even
fn is_even(n: i32) -> bool {
    let mut flag: bool;

    if n % 2 == 1 {
        flag = false;
    }else{
        flag = true;
    }

    flag
}