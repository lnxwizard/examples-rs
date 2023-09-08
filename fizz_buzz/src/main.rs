/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flow (If-Else, for loop)
*/

fn main() {
    fizz_buzz();
}

fn fizz_buzz() {
    for i in 0..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
            continue;
        } else if i % 3 == 0 {
            println!("Fizz");
            continue;
        } else if i % 5 == 0 {
            println!("Buzz");
            continue;
        }

        println!("{i}");
    }
}
