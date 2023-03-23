use std::{env, fs};

fn get_cmd_args () -> (String, usize) {
    // Get the command-line arguments and store them in a vector of strings
    let args: Vec<String> = env::args().collect();

    // Extract the second command-line argument, which should be the path to the file to read
    let file_path = args[1].to_string();

    // Extract the third command-line argument, which should be number of top numbers you want to output
    let n= args[2].parse::<usize>().unwrap();

    // Return `file_path` and `n` variables
    (file_path, n)
}

fn top_n_nums (mut numbers: Vec<i32>, n: usize) ->  () {
    // Sort `numbers`
    numbers.sort();

    // Reverse the sorting on `numbers` so that it's listed in descending order
    numbers.reverse();

    // Set the `nums_slice` variable to a slice of `numbers`
    let nums_slice = numbers.as_slice();

    // Print line with the `n` values, and then loop through `n` times through the sorted slice
    println!("Top {} values from file:", n);
    for i in &nums_slice[..n] {
        println!("{}", i);
    }
}

fn main() {
    
    // Assign `file_path` and `n` variables the return values from get_cmd_args
    let (file_path, n): (String, usize) = get_cmd_args();

    // Print a message indicating the file that will be read
    // println!("In file {}", file_path);

    // Read the file's contents and store them in `contents` variable as a String.
    // The `.expect()` line allows for error handling if the file is unable to be read for any reason
    let contents: String = fs::read_to_string(file_path)
        .expect("Unable to read file.");

    // Print the contents of the file to the console
    // println!("With text:\n{contents}");

    // Initialize a mutable vector named `numbers` of type `Vec<i32>`
    let numbers: Vec<i32> = 

        // Trim any whitespace from the beginning and end of the string
        contents.trim()

        // Split the string into a vector of substrings wherever a comma is found
        .split(",")
    
        // Map each substring to a parsed integer using the `parse()` method, which returns a `Result` type
        // `unwrap()` is called on the `Result` type to convert it to an `i32`
        .map(|s| s.parse::<i32>().unwrap())

        // Collect the parsed integers into a `Vec<i32>` and assign them to the `numbers` variable
        .collect();

    top_n_nums(numbers, n);





}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_add() {
    //     assert_eq!(add(1, 2), 3);
    // }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }

    #[test]
    fn zero_args() {
        get_cmd_args();
    }

    // fn nonexistant_file() {
    //     let no_file: String = String::("thisfiledoesnotexist");
    //     get_cmd_args();
    // }

}