/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flow (If, for loop)
    - Functions
*/


use std::io;

fn main() {
    // get first string
    println!("Enter first string:");
    let mut first_string= String::new();
    io::stdin()
        .read_line(&mut first_string)
        .expect("Failed to get first string!");

    // get first string
    println!("Enter second string:");
    let mut second_string= String::new();
    io::stdin()
        .read_line(&mut second_string)
        .expect("Failed to get second string!");

    // print result
    println!("'{}' and '{}' are anagrams? -> {}", first_string.trim(), second_string.trim(), are_anagrams(first_string.clone(), second_string.clone()));
}

// find anagram
fn are_anagrams(first_string: String, second_string: String) -> bool {
    // if length of two string are not equal then return false
    if first_string.trim().len() != second_string.trim().len() {
        return false;
    }

    // define same word count
    let mut same_words = 0;

    // count same words
    for s1_letter in first_string.trim().to_lowercase().chars() {
        for s2_letter in second_string.trim().to_lowercase().chars() {
            if s2_letter == s1_letter {
                same_words += 1;
            }
        }
    }
    
    if same_words != first_string.trim().len() {
        return false;
    }

    return true;
}
