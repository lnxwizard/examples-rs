/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Functions
*/

use std::io;

fn main() {
    // get number from user
    println!("Enter a number:");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to get number!");
    // convert String to u32
    let number: u32 = number.trim().parse().expect("Enter a number!");

    // print result
    println!(
        "Is {} palindrome number? -> {}",
        number,
        is_palindrome_number(number)
    );
}

fn is_palindrome_number(n: u32) -> bool {
    // convert number to String and reverse characters
    let reversed_number: String = n.to_string().chars().rev().collect::<String>();
    // convert reversed number to u32
    let reversed_number: u32 = reversed_number.trim().parse().unwrap();

    // return result
    n == reversed_number
}
