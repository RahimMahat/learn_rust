use std::fs::File;
use std::process;

fn file_handling() {
    // Opening a file.
    let file = match File::open("notes.txt") {
        Ok(file) => file,
        Err(err) => {
            // Prints to the standard error, while println! prints to the standard output, with a new line.
            eprintln!("Something went wrong, error: {err:?}");
            // exit the program gracefully with an error code without panicking.
            process::exit(1);
        }
    };
    println!("{file:#?}");
}

fn main() {
    file_handling();
}
