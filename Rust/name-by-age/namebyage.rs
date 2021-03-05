use std::io::{self, Write};

fn main() {
  // Obtain name
  print!("Enter name: ");
  io::stdout().flush().unwrap();
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Failed to obtain name");
  
  // Obtain age
  print!("Enter age: ");
  io::stdout().flush().unwrap();
  let mut age = String::new();
  io::stdin().read_line(&mut age).expect("Failed to obtain age");
  age = age.to_string();

  // Print result
  println!("");
  for i in 0..(age.trim().parse::<u32>().unwrap()) {
    println!("{}", name.trim());
  }
}