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
    // getting string from user
    println!("Enter a word:");
    let mut words = String::new();
    io::stdin()
        .read_line(&mut words)
        .expect("Failed to get String!");

    // print reversed string
    println!("{} -> {}", words.trim(), reverse_words(words.to_string()));
}

/*
* this function reverse strings
* example: abc -> cba
*/
fn reverse_words(s: String) -> String {
    s.trim().chars().rev().collect::<String>()
}
