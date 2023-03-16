use std::{env, fs};

fn main() {
    // Get the command-line arguments and store them in a vector of strings
    let args: Vec<String> = env::args().collect();

    // Extract the second command-line argument, which should be the path to the file to read
    let file_path = &args[1];

    // Extract the third command-line argument, which should be number of top numbers you want to output
    let n= args[2].parse::<i32>().unwrap();

    // dbg!(&n);

    // Print a message indicating the file that will be read
    // println!("In file {}", file_path);

    // Read the file's contents and store them in a string
    let contents: String = fs::read_to_string(file_path)
        .expect("Unable to read file.");

    // Print the contents of the file to the console
    // println!("With text:\n{contents}");

    // Initialize a mutable vector named `numbers` of type `Vec<i32>`
    let mut numbers: Vec<i32> = 

        // Trim any whitespace from the beginning and end of the string
        contents.trim()

        // Split the string into a vector of substrings wherever a comma is found
        .split(",")
    
        // Map each substring to a parsed integer using the `parse()` method, which returns a `Result` type
        // `unwrap()` is called on the `Result` type to convert it to an `i32`
        .map(|s| s.parse::<i32>().unwrap())

        // Collect the parsed integers into a `Vec<i32>` and assign them to the `numbers` variable
        .collect();

    // dbg!(&numbers);

    // Sort `numbers`
    numbers.sort();
    
    // dbg!(&numbers);

    // Reverse the sorting on `numbers` so that it's listed in descending order
    numbers.reverse();
    
    // dbg!(&numbers);

    // Set the `slice` variable to a slice of `numbers`
    let slice = numbers.as_slice();
    
    // dbg!(&slice);

    // Convert `n` from an `i32` type to `usize` type, because that seems to work
    let n: usize = n as usize;

    // Print line with the `n` values, and then loop through `n` times through the sorted slice
    println!("Top {} values from file:", n);
    for i in &slice[..n] {
        println!("{}", i);
    }

}
