use std::fs::File;
use std::io::{Read, stdin};
use std::process;

fn file_handling() {
    // Opening a file.
    println!("Enter filename to open: ");
    let mut input = String::new();
    let user_requested_file = stdin().read_line(&mut input);

    if let Err(err) = user_requested_file {
        eprintln!("Something went wrong getting user input: Error: {err}");
        process::exit(1)
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(err) => {
            // Prints to the standard error, while println! prints to the standard output, with a new line.
            eprintln!("Something went wrong opening the file, error: {err}");
            // exit the program gracefully with an error code without panicking.
            process::exit(1);
        }
    };

    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);
    if let Err(err) = read_operation {
        eprintln!("Something went wrong reading the file: error: {err}");
        process::exit(1)
    }
    println!("{file_contents}");
}

fn main() {
    file_handling();
}
