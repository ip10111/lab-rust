// Convert decimal to binary

use std::vec::Vec;

fn main() {
    let d: u32 = 23;

    println!("{}", decimal_to_binary_ver_arr(d));

    println!("{}", decimal_to_binary_ver_str(d));
}

// Array version: collect the sequence in an Array, then convert it to string
fn decimal_to_binary_ver_arr(mut decimal: u32) -> String {
    let mut end: bool = false;
    let mut remainder: u32;
    let mut binary_sequence: Vec<u32> = Vec::new();

    while !end {
        remainder = decimal % 2;
        decimal /= 2;

        if decimal == 0 {
            end = true
        }

        binary_sequence.insert(0, remainder);
    }

    binary_sequence
        .iter()
        .map(|&x| x.to_string())
        .collect::<String>()

    // Convert each element of the array to a string. using map method and
    // the to_string method. The resulting iterator of strings is collected
    // into a Vec<String>, and then the join method is
    // called on it to concatenate the elements.
    // binary_sequence
    //     .iter()
    //     .map(|&x| x.to_string())
    //     .collect::<Vec<String>>()
    //     .join("")
}

// String version: convert directly to String
fn decimal_to_binary_ver_str(mut decimal: u32) -> String {
    let mut remainder: u32;
    let mut binary_sequence: String = String::new();

    while decimal > 0 {
        remainder = decimal % 2;
        decimal /= 2;

        binary_sequence.insert(0, char::from_digit(remainder, 10).unwrap());
    }
    binary_sequence
}
