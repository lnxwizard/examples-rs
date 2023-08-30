/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flow (If-Else)
    - Functions
*/

use std::io;

fn main() {
    println!("What Fibonacci number do you want to find?");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to get number!");
    let n: i32 = n.trim().parse().expect("Only numbers allowed!");

    println!("{n}th Fibonacci number is: {}", fib(n - 1));
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        // return `n`
        n
    } else if n == 1 {
        // return `n`
        n
    } else {
        // return n(th) fibonacci number
        fib(n - 1) + fib(n - 2)
    }
}
