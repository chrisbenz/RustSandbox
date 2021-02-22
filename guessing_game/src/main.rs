use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // println!("Input a number between 1 and 100 to designate your guess: ");
    let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("The number to guesss was: {}", random_num);
    // let mut guess = String::new(); // Guess is mutable 
    
    // io::stdin()
    //    .read_line(&mut guess)
    //    .expect("Failed to read line");

    // println!("You guessed {}", guess);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line from stdin");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed the number: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
