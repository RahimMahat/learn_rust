use std::fs::File;
use std::io::{self, Read, stdin};

fn file_handling() -> Result<String, io::Error> {
    // Opening a file.
    println!("Enter filename to open: ");
    let mut input = String::new();
    let user_requested_file = stdin().read_line(&mut input);

    if let Err(err) = user_requested_file {
        return Err(err);
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(err) => {
            // Prints to the standard error, while println! prints to the standard output, with a new line.
            // eprintln!("Something went wrong opening the file, error: {err}");
            // exit the program gracefully with an error code without panicking.
            // process::exit(1);
            return Err(err);
        }
    };

    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);
    if let Err(err) = read_operation {
        return Err(err);
    }

    Ok(file_contents)
}

fn main() {
    let file_result = file_handling();
    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}");
        }
    }
}
