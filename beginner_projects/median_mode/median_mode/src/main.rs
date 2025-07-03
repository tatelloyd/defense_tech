use std::io;

fn main() {
    // Initialize a vector to hold the user input.
    let mut v: Vec<i32> = vec![];

    // Main Loop 
    loop {
    
        // Prompt user for  input.
        println!("\nPlease enter a number to add it to the vector, type 'median' to display the median, 
        type 'mode' to display the mode, 'vector' to see the vector, or type 'exit' to quit):");

        // Create variable for holding input.
        let mut input = String::new();

        // Read the user input.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // Trim whitespace and check for valid input.
        if input.trim().contains(char::is_whitespace) || input.trim().is_empty(){
            println!("Invalid input. Please enter a number, 'median', 'mode', 'vector', or 'exit'.");
            continue;
        // Check if input is a number and add it to the vector. Display the updated vector.
        } else if input.trim().parse::<i32>().is_ok() {
            v.push(input.trim().parse::<i32>().unwrap());
            println!("The new vector is: {:?}", v);
        // Check if input contains any numeric characters
        } else if input.chars().any(|input| input.is_numeric()){
            v.push(input.trim().parse::<i32>().unwrap());
            println!("The new vector is: {:?}", v);
        // Return the median of the vector
        } else if input.trim() == "median" {
            let median_value = median(&v);
            println!("The median of the vector is: {}", median_value);
        // Return the mode of the vector
        } else if input.trim() == "mode" {
            let mode_value = mode(&v);
            println!("The mode of the vector is: {}", mode_value);
        // Display the current vector
        } else if input.trim() == "vector" {
            println!("The current vector is: {:?}", v);
        // Exit the program
        } else if input == "exit" {
            println!("Exiting the program.");
            break;
        // Handle invalid input
        } else {
            println!("Invalid input. Please enter a number, 'median', 'mode', 'vector', or 'exit'.");
        }

    }

}
    
// Function to calculate the median of a vector of integers.
fn median(v: &Vec<i32>) -> f64 {
    // Copy the vector and sort it.
    let mut v = v.clone();
    if v.is_empty() {
        return 0.0; // Return 0.0 if the vector is empty
    }
    v.sort();
    
    // Calculate the median based on the length of the vector.
    let len = v.len();
    // If the length is even, return the average of the two middle elements.
    if len % 2 == 0 {
        (v[len / 2 - 1] + v[len / 2]) as f64 / 2.0
    // If the length is odd, return the middle element.
    } else {
        v[len / 2] as f64
    }
}

// Function to calculate the mode of a vector of integers.
fn mode(v: &Vec<i32>) -> i32 {
    // Use a HashMap to count occurrences of each value in the vector.
    use std::collections::HashMap;

    if v.is_empty() {
        return 0; // Return 0 if the vector is empty
    }

    // Create a HashMap to store occurrences of each value.
    let mut occurrences = HashMap::new();
    // Iterate through the vector and count occurrences of each value.
    for &value in v {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    // Find the value with the maximum occurrences.
    // If there are multiple modes, return the first one found.
    occurrences.into_iter().max_by_key(|&(_, count)| count).map_or(0, |(value, _)| value)
}