/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flows (if, for loop)
    - Functions
*/

use std::io;

fn main() {
    // get number input from user
    println!("Enter a number:");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to get number!");
    // convert String to u32 (32-bit unsigned integer)
    let num: u32 = num.trim().parse().expect("Enter a number!");

    // print the result
    println!("Is {num} perfect number? -> {}", perfect_number(num));
}

// find perfect numbers with this function
fn perfect_number(n: u32) -> bool {
    let mut sum: u32 = 0;

    for i in 1..n {
        if n % i == 0 {
            sum = sum + i;
        }
    }

    // if `n` is perfect number retun true, else return false
    n == sum
}
