// Adder Function
fn add(x:i32, y:i32) -> i32{
    // Add the two numbers
    return x + y;
}

// Main function
fn main(){
    // Declare variables
    let mut _z = 0;
    let a = 1;
 
    //Call the function
    _z = add(a, a);

    //Return the output
    println!("{}", _z);
}