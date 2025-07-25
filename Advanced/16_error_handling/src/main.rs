use std::io::{self, stdin};
use std::{fs, process};

fn read_file() -> Result<String, io::Error> {
    // Opening & Reading a file.
    println!("Enter filename to open: ");
    let mut input = String::new();
    // you can use the '?' try operator on a function/method that returns Result,
    // It'll return the Ok variant if the function gives us Ok and continue with rest of the program but it'll return Err variant if the function gives us error.
    stdin().read_line(&mut input)?;

    // The '?' operator can only be used in a function that returns 'Result' or 'Option' (or another type that implements 'FromResidual')

    // There is an easier and efficent way of achieving the below 3 lines in one single line. the
    // fs module has a function called read_to_string that only takes the file path reads the file
    // content and returns it in Ok variant or Err variant if error.
    // let mut file_contents = String::new();
    // this read_to_string function is from the io module.
    // fs::File::open(input.trim())?.read_to_string(&mut file_contents)?;
    // Ok(file_contents)

    // The above 3 lines replaced with this just one line
    // this read_to_string is from fs module which returns the Result packaged variant
    fs::read_to_string(input.trim())
}

fn write_to_file() -> io::Result<String> {
    println!("What file would you like to write to? ");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name)?;

    println!("What would you like to write in the file? ");
    let mut content = String::new();
    stdin().read_line(&mut content)?;

    fs::write(file_name.trim(), content.trim())?;

    Ok(file_name)
}

fn main() {
    let file_result = read_file();
    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}");
            process::exit(1)
        }
    }

    match write_to_file() {
        Ok(file_name) => println!("Successfully wrote to file {file_name}"),
        Err(error) => {
            eprintln!("Error writing to file: {error}");
        }
    }
}
