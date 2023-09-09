/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flow (If-Else, loop)
    - Crates (rand)
*/

use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Guess the Number Game!");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Guess a number between 0-100: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get number!");
        let guess: u8 = guess.trim().parse().expect("Only numbers allowed!");

        if guess == secret_number {
            println!("You Win!");
            break;
        } else if guess < secret_number {
            println!("Bigger");
            continue;
        } else if guess > secret_number {
            println!("Lower");
            continue;
        }
    }
}
