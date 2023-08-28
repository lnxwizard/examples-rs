/*
    Author: Alper Akça (lnxwizard) <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flow (If-Else)
    - Functions
*/

use std::io;
use std::process;

// main function
fn main() {
    // getting first number from user
    println!("Enter first number:");
    let mut n1 = String::new();
    io::stdin()
        .read_line(&mut n1)
        .expect("Failed to get first number!");
    let n1: i32 = n1.trim().parse().expect("Please enter a number!");

    // getting second number from user
    println!("Enter second number:");
    let mut n2 = String::new();
    io::stdin()
        .read_line(&mut n2)
        .expect("Failed to get second number!");
    let n2: i32 = n2.trim().parse().expect("Please enter a number!");

    // getting operation from user
    println!("Select operation: \n1: Addition \n2: Subtraction \n3: Division \n4: Multiplication \n5: Remainder");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to get operation!");
    let operation: u8 = operation.trim().parse().expect("Please enter a number!");

    // printing the result
    if operation == 1 {
        println!("{n1}+{n2} = {}", addition(n1, n2));
    } else if operation == 2 {
        println!("{n1}-{n2} = {}", subtraction(n1, n2));
    } else if operation == 3 {
        println!("{n1}/{n2} = {}", division(n1, n2));
    } else if operation == 4 {
        println!("{n1}*{n2} = {}", multiplication(n1, n2));
    } else if operation == 5 {
        println!("{n1}%{n2} = {}", remainder(n1, n2))
    } else {
        // if the user enters an invalid option print error message and exit
        println!(
            "\n{} is not a valid option! \n Please enter a number between 1-5",
            operation
        );
        process::exit(2);
    }
}

/* `addition` function returns the sum of two numbers.
 *
 * Parameters
 * n1: first number
 * n2: second number*/
fn addition(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

/* `subtraction` function returns the difference of two numbers.
 *
 * Parameters
 * n1: first number
 * n2: second number*/
fn subtraction(n1: i32, n2: i32) -> i32 {
    n1 - n2
}

/* `division` function returns the quotient of two numbers.
 *
 * Parameters
 * n1: first number
 * n2: second number */
fn division(n1: i32, n2: i32) -> i32 {
    n1 / n2
}

/* `multiplication` function returns the product of two numbers.
 *
 * Parameters *
 * n1: first number
 * n2: second number */
fn multiplication(n1: i32, n2: i32) -> i32 {
    n1 * n2
}

/* `remainder` function returns the remainder of two numbers.
 *
 * Parameters
 * n1: first number
 * n2: second number */
fn remainder(n1: i32, n2: i32) -> i32 {
    n1 % n2
}
