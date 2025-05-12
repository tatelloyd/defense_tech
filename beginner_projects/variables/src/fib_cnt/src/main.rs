// This program prompts the user for a number
// input and returns the corresponding spot in
// Fibonacci Sequence (i.e. 3 returns 3rd spot or 2).

// Included libraries
use std::io;

fn main() {   

    // Loop for taking in inputs
    loop {

        // Create variable for holding the desired index
        let mut index  = String::new();

        // User prompt
        println!("Please enter which index of the Fibonacci Sequence you'd like to see. Enter 'Exit' at anytime to quit the program");

        // Read the index input
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        // Code block to escape the program
        if index.trim() == "Exit"
        {
            break;
        }

        // Handle invalid inputs
            let index: u128 = match index.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        // First two Fibonnaci values.
        // These update continuously
        let mut fib_curr: u128 = 1;
        let mut fib_prev: u128 = 1;

        // Loop for getting nth fibonnaci value.
        // Skip the first two iterations since those
        // values don't need to be calculated.
        for x in 1..index - 1 {
                fib_curr += fib_prev;
                fib_prev = fib_curr - fib_prev;
        }

        // Print the output
        println!("{fib_curr}");
    }    
}