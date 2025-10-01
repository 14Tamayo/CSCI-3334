use std::io;

fn main() {
    // Variable definiton
    let mut iSecret: i32 = 44;
    let mut iGuess: i32;
    let mut iResult: i32;
    let mut iCounter: i32 = 0;
    

    loop {
        //Input variable
        let mut sInput = String::new();

        //Prompt & input
        println!("Enter your guess.");
        io::stdin()
            .read_line(&mut sInput)
            .expect("Failed to read input");

        //Parsing input into an integer
        iGuess = sInput
            .trim()
            .parse()
            .expect("Invalid input: Not an integer");


        //Call function & store results
        iResult = check_guess(iGuess, iSecret);
        iCounter += 1;

        //Check results
        if iResult == 1 {
            println!("Too high")
        }else if iResult == -1 {
            println!("Too low");
        }
        if iResult == 0 {
            break;
        }
    }
    
    //Clear screen
    print!("\x1b[2J\x1B[1;1H");
    
    //Output results
    println!("{iSecret} Is the is the correct number.");
    println!("It took you {iCounter} tries to get it.")


}


//Function
fn check_guess(guess: i32, secret: i32) -> i32 {

    if guess == secret {
        0
    }else if guess > secret {
        1
    }else{
        -1
    }
}