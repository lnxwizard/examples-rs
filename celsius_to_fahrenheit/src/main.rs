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
use std::process;

fn main() {
    // print available operations
    println!("Please select a operation below:\n");
    println!("1: Celsius to Fahrenheit\n2: Fahrenheit to Celsius\n3: Exit\n");

    // getting the user's input for the operation
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to get operation!");
    let operation: u8 = operation.trim().parse().expect("Enter number!");

    // processing the user's input
    if operation == 1 {
        println!("Result: {}", celsius_to_fahrenheit())
    } else if operation == 2 {
        println!("Result: {}", fahrenheit_to_celsius())
    } else if operation == 3 {
        process::exit(0);
    } else {
        println!("{operation}: is a not valid option!")
    }
}

// this function converts Celsius to Fahrenheit.
fn celsius_to_fahrenheit() -> f64 {
    println!("Enter Celsius Value:");
    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to get Celsius value!");
    let celsius: f64 = celsius.trim().parse().expect("Enter number!");

    celsius * 1.8000 + 32.0
}

// this function converts Fahrenheit to Celsius.
fn fahrenheit_to_celsius() -> f64 {
    println!("Enter Fahrenheit Value:");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to get Fahrenheit value!");
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Enter number!");

    (fahrenheit - 32.0) / 1.8000
}
