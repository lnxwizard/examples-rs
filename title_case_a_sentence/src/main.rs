/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types
    - Functions
*/

fn main() {
    // define strings
    let hello: &str = "hElLo";
    let rustaceans: &str = "ruStACeaNs";

    // print message
    println!("{}, {}!", title_case(hello), title_case(rustaceans));
}

// Make first letter of each word capitalized and rest lower case
fn title_case(s: &str) -> String {
    // return result
    s[0..1].to_uppercase() + &s[1..].to_lowercase()
}
