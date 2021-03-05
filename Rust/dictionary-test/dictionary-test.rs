use std::collections::HashMap;

fn main() {
    // Make simple dictionary
    let mut alphabet = HashMap::new();

    alphabet.insert('A', 1);
    alphabet.insert('B', 2);
    alphabet.insert('C', 3);
    alphabet.insert('D', 4);
    alphabet.insert('E', 5);

    for (letter, num) in &alphabet {
        println!("{}::{}", letter, num);
    }
    println!("{}", alphabet);
}