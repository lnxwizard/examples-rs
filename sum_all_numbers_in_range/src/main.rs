/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Control Flow (for loop)
    - Functions
*/

fn main() {
    // define number one and number two
    let (n1, n2): (i32, i32) = (6, 12); // == 6 + 7 + 8 + 9 + 10 + 11 + 12

    // print result
    println!("{}", sum_all(n1, n2));
}

// sum all numbers in range
fn sum_all(n1: i32, n2: i32) -> i32 {
    // define sum
    let mut sum: i32 = 0;

    // sum all numbers in range
    for i in n1..=n2 {
        sum += i;
    }

    // return sum
    sum
}
