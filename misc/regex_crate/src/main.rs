use regex::Regex;

fn main() {
    // the find and find_iter methods
    let re = Regex::new(r"ue").unwrap();
    let text = "my movie queue";

    for data in re.find_iter(text) {
        println!("{} {} {}", data.start(), data.end(), data.as_str());
    }

    match re.find(text) {
        Some(data) => {
            println!("{:?}", data);
        }
        None => println!("No matches found."),
    }

    // Searching for digits
    let text = "My ZIP Code is 83332.";
    let digit_re = Regex::new(r"\d").unwrap();
    for digit in digit_re.find_iter(text) {
        print!("{:?}", digit.as_str())
    }

    // Alphanumeric characters
    let alphanumeric_re = Regex::new(r"\w").unwrap();
    for alnum in alphanumeric_re.find_iter(text) {
        print!("{:?}", alnum.as_str())
    }

    // Whitespaces
    // let whitespace_re = Regex::new(r"\s").unwrap();
    let inverse_whitespace_re = Regex::new(r"\S").unwrap();
    for whitespace in inverse_whitespace_re.find_iter(text) {
        print!("{:?}", whitespace.as_str())
    }

    // Word boundries - \b
    let starting_with_c_word_boundries_re = Regex::new(r"\bC").unwrap();
    for boundry in starting_with_c_word_boundries_re.find_iter(text) {
        print!("{:?}", boundry)
    }

    // The dot metacharacter
    let i_and_anything_after_re = Regex::new(r"i.").unwrap();
    for i in i_and_anything_after_re.find_iter(text) {
        println!("{:?}", i)
    }

    // square brackets with characters
    let r_to_z_match = Regex::new(r"\b[A-Dr-z]").unwrap();
    for rz in r_to_z_match.find_iter(text) {
        print!("{:?}", rz)
    }

    // Capture groups
    let text = "123 Palm Street, Elm Springs, CA";
    let re = Regex::new(r"(?<street_number>\d+).+(?<state>\w{2})").unwrap();
    let captures = re.captures(text).unwrap();
    println!("\n{}", &captures[0]);
    println!("{}", &captures["street_number"]);
    println!("{}", &captures["state"]);

    // the replace_all method
    let text = "I have 2 apples and 10 bananas";
    let replace_re = Regex::new(r"(?<count>\d+)\s(?<fruit>\w+)").unwrap();
    let result = replace_re.replace_all(text, "$fruit ($count)");
    println!("{result}");
}
