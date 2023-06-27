// Write a function that will take a given string and reverse the order of the words.
use std::io;
fn main() {
    println!("Enter the sentence ....");
    let mut setntence = String::new();
    io::stdin()
        .read_line(&mut setntence)
        .expect("Invalid entry.");
    println!("Entered sentence is \n{}", setntence);
    reverse_string(&mut setntence);
}
fn reverse_string(sent: &mut String) {
    let op = sent
        .split_ascii_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ");
    println!("outuput is \n {:?}", op);
}
