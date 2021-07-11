use std::io;
use std::process;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    read_line();
}

fn read_line() {
    // choose number from stdin input
    println!("Guess the number!");
    println!("Please input your guess : ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("You guessed: {} ", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! , the secret number is {}", secret_number),
        Ordering::Greater => println!("Too big! , the secret number is {}", secret_number),
        Ordering::Equal => println!("You win!, the secret number is {}", secret_number),
    }

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
        println!("Goodbye!");
        process::exit(1);
    }
}