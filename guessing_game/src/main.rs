use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("Please input your guess:");

        // mut means mutable so can be changed
        // let apples = 5; this one you can not change
        // let mut apples = 5; this one can change.
        let mut guess = String::new();

        //read_line() appends the text!
        //read_line returns Result enum.
        // Variations are Ok and Err.
        // if it is Err .expect will run.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        //converts string to u32 (unsigned 32 bit number) to compare it
        // Shadowing the previous value.
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    //let x = 5;
    //let y = 10;

    //println!("x = {x} and y + 2 = {}", y + 2);
}
