// Program allows for conversion from 
// Fahrenheit to Celsius and vice-versa.

// Included libraries
use std::io;

fn main() {

    // Main loop. This layer deals with the main menu prompt.
    'calculator: loop {
        // Prompt user for unit input
        println!("Convert to F, to C, or exit the program?");

        // Create variable for holding the unit
        let mut unit  = String::new();

        // Read the unit input
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");
        
        // Inner loop for taking the value input
        'value: loop {
            // Convert the temperature from Fahrenheit to Celsius
            if unit.trim() == "C" {
                calc_handler(&unit, &f_to_c);
            }
            // Convert the temperature from Celsius to Fahrenheit
            else if unit.trim() == "F" {
                calc_handler(&unit, &c_to_f);      
            }
            //Exit the program if prompted
            else if unit.trim() == "exit" {
                break 'calculator;
            }
            // Reinforce the menu options
            else {
                println!("Please enter C, F, or exit!");
            }
            // Break from inner loop to return to original menu
            break 'value;
        }
    }
}

// Fahrenheit to Celsius
fn f_to_c(x: f32) -> f32 {
    return (x - 32.0) * (5.0/9.0)
}

// Celsius to Fahrenheit
fn c_to_f(x: f32) -> f32 {
    return x * (9.0/5.0) + 32.0
}

// Function processing value inputs and outputting 
fn calc_handler(unit: &str, f: &dyn Fn(f32) -> f32) {
    loop {
        // Prompt the user for a value
        println!("Value?");

        // Create a variable to hold the inputted variable
        let mut value = String::new();

        // Read the user input
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        // Handle invalid inputs
        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Make the proper conversion and print the result
        if unit.trim() == "C"
        {
            println!("{value} in Fahrenheit is {c} in Celsius", c = f(value));
        }
        else if unit.trim() == "F"
        {
            println!("{value} in Celsius is {f} in Fahrenheit", f = f(value));
        }

        // Exit Loop once a proper conversion has been made.
        break; 
   }
}


