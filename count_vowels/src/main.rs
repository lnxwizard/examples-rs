/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flow (If-Else, for loop)
    - Functions
*/

use std::io;

fn main() {
    // get string input
    println!("Enter a string:");
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to get string!");

    // print result
    println!("We found {} vowels in `{}`", count_vowels(string.clone()), string.trim());
}

// find vowels in string
fn count_vowels(s: String) -> u32 {
    // define vowels
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut count = 0;

    // count vowels
    for letter in s.trim().to_lowercase().chars() {
        if VOWELS.contains(&letter) {
            count += 1;
        }
    }

    // return amount of vowels in string
    count
}
