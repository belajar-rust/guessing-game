use std::io;
use std::process;

fn main() {
    read_line();
}

fn read_line() {
    // choose number from stdin input
    println!("Guess the number!");
    println!("Please input your guess : ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {} ", guess);
    println!("continue ?(1. yes, 2. no): ",);

    // input string from stdin input
    let mut choice = String::new();

    // scan the input
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    // and parse to integer (i32)
    let input_num: i32 = choice.trim().parse().expect("Not a valid number");

    //set get back to function itself(recursive) or exit the process(program)
    if input_num == 1 {
        return read_line();
    } else {
        process::exit(1);
    }
}