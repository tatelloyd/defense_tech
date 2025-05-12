// This program prints out the 12 Days of Chrristmas
// song leveraging arrays.

fn main() {
   
   // Lyrics
   let lyrics = ["a partridge in a pear tree", 
                "two turtle doves", "three french hens",
                "four calling birds", "five golden rings",
                "six geese a laying", "seven swans a swimming",
                "eight maids a milking", "nine ladies dancing",
                "ten lords a leaping", "eleven pipers piping",
                "twelve drummers drumming"];

   // Suffixes
   let suffixes = ["1st", "2nd", "3rd", "4th", "5th", "6th",
                    "7th", "8th", "9th", "10th", "11th", "12th"];

    // Loop for 12 days.                
    for i in 0..lyrics.len() {
        // Print first part of the chorus
        println!("\nOn the {} day of Christmas my true love gave to me,", suffixes[i]);

        // Reverse back from the ith day to the first days
        for j in (0..i+1).rev() {
            
            // If it is the first day, end it with a period
            if i == 0 {
                println!("{}.", lyrics[j]);
            }
            // If it is the end of any day other than the first day,
            // end with an and to get the grammar of the song correct.
            else if j == 0 && i > 0 {
                println!("and {}.", lyrics[j]);
            }
            // Otherwise, just print out the lyric for the gift on the given day.
            else {
                println!("{},", lyrics[j]);
            }
        }
    }
}
