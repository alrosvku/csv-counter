use std::{env, fs};


fn get_cmd_args (mut args_iter: impl Iterator<Item = String>) -> Result<(String, usize), String> { 
    //let arg1: Option<String> = args.nth(1);

    let file_path = args_iter
        .next()
        .ok_or("No arguments passed. Please enter a filepath and number as command arguments.".to_string())?;

    let n_str = args_iter
        .next()
        .ok_or("Second argument not provided. Please enter a number for the second argument.".to_string())?;

    let n = n_str
        .parse::<usize>()
        .map_err(|_| "Incorrect second argument. Must be a number".to_string())?;

    Ok((file_path, n))

}


fn get_file_contents (file_path: String) -> String {
    // Read the file's contents and store them in `contents` variable as a String.
    // The `.expect()` line allows for error handling if the file is unable to be read for any reason
    let contents: String = fs::read_to_string(file_path)
        .expect("Unable to read file.");

    // Print the contents of the file to the console
    // println!("With text:\n{contents}");
    contents
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

    let args: env::Args = env::args();

    let args_iter = args.skip(1);

    let (file_path, n) = match get_cmd_args(args_iter) {
        Ok((file_path, n)) => {
            (file_path, n)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            (String::new(), 0)
        }
    };

    // Print a message indicating the file that will be read
    println!("In file {}", file_path);

    let contents: String = get_file_contents(file_path);

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


// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_args_zero_cmd_args() {
        let args: [&str; 3] = [""; 3];
        let mut args_iter = args
            .iter()
            .map(|&s| s.to_string());
        // let (file_path, n) = get_cmd_args(args_iter);
        
    }

    // #[test]
    // fn get_args_empty_cmd_args() {
    //     let args: Vec<String> = vec![String::new(); 3];
    //     get_cmd_args(args);
    // }

    // #[test]
    // fn get_args_wrong_order_args() {
    //     let args: Vec<String> = vec![
    //         String::from("path_of_executable"),
    //         String::from("1"),
    //         String::from("csv-file.csv")
    //     ];
    //     get_cmd_args(args);
    // }

    // #[test]
    // #[should_panic]
    // fn get_args_nonexistant_file() {
    //     let args: Vec<String> = vec![
    //         String::from("path_of_executable"),
    //         String::from("thisfiledoesnotexist"),
    //         String::from("1")
    //     ];
    //     get_cmd_args(args);
    // }

    // #[test]
    // fn get_contents_missing_file() {
    //     let file_path: String = String::from("thisfiledoesnotexist");
    //     get_file_contents(file_path);
    // }

}