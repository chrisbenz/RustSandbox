use std::io;

fn main() {
    println!("Hello, world!");

    println!("Input a number to designate your guess: ");

    let mut guess = String::new(); // Guess is mutable 
    
    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");

    println!("You guessed {}", guess);
}
