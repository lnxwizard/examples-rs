/*
    Author: Alper Akça <alperakca79@outlook.com>
    License: GPLv3

    Requirements:

    I recommend completing the following be completed before beginning this example:
    - Variables and Mutability
    - Data Types (array, &str, integers)
    - Control Flow (for loop)
    - Functions
*/

fn main() {
    let days: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "FIVE GOLD RINGS",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        let suffix: &str = match day + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };

        println!(
            "\nOn the {}{} day of xmas my true love gave to me:",
            day + 1,
            suffix
        );
        for item in (0..day + 1).rev() {
            if day > 0 && item == 0 {
                print!("And ");
            }
            println!("{}", days[item]);
        }
    }
}
