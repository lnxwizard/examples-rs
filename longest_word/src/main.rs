/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flows (if-else)
    - Functions
*/

fn main() {
    // define array
    let arr: [&str; 3] = ["Rust", "Programming", "Language"];

    // find the longest string in array
    find_longest_word(arr);
}

// find the longest string in array
fn find_longest_word(arr: [&str; 3]) {
    if arr[0].len() > arr[1].len() && arr[0].len() > arr[2].len() {
        println!("Longest string in array is: {}[{}]", arr[0], arr[0].len());
    } else if arr[1].len() > arr[0].len() && arr[1].len() > arr[2].len() {
        println!("Longest string in array is: {}[{}]", arr[1], arr[1].len());
    } else {
        println!("Longest string in array is: {}[{}]", arr[2], arr[2].len())
    }
}
