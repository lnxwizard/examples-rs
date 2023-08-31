/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flow (If && `for` loop)
    - Functions
*/

use std::io;

fn main() {
    // getting number from user
    println!("Enter a number:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to get number!");
    // convert String to i32
    let n: i32 = n.trim().parse().expect("Enter a number!");

    // print result
    println!("{n}! = {}", factorial(n));
}

// this functions returns factorial of number
fn factorial(n: i32) -> i32 {
    let mut result: i32 = 1;

    if n == 0 {
        result = 1;
    }

    for i in (1..n + 1).rev() {
        result *= i;
    }

    result
}
