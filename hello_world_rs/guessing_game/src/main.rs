<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
=======
// Import the needed libraries
use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    // Prompt the user
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Create string to store user input
        let mut guess = String::new();

        //Store user input with error handler
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
>>>>>>> Finished Guessing Game
}
