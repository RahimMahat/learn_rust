/*
 This package gives you is for the practice & overview of strings in Rust.
*/
use std::io;

fn user_input() {
    let mut name = String::new();
    println!("What is your name? ");
    let _ = io::stdin().read_line(&mut name); // the user input will be captured in the name
    // variable since we're passing the heap allocated mutable string reference and hence we can interpolate that var in the next line.
    println!("Hello, {}", name.trim());
}

fn common_string_methods() {
    let mut music_genres = "      Rock, Metal, Country, Rap       ";
    println!("{}", music_genres.trim()); // trims leading and trailing whitespaces
    println!("{}", music_genres.trim_start()); // trim_start trims leading whitespaces
    println!("{}", music_genres.trim_end()); // trim_start trims trailing whitespaces
    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("a", "@"));

    let genre: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genre);
}

fn format_macro() {
    let first_name = "Peter";
    let last_name = "Parker".to_string();
    let icon = format!("{} {}", first_name, last_name); // format macro instead of printing a string
    // rather returns the formatted string, without taking ownership.
    println!("{icon}");
}

fn concatination() {
    let mut full_name = String::from("Andrew");
    let last_name = "Garfield";
    full_name.push(' '); // push expects a char to concatenate
    full_name.push_str(last_name); // push_str expects a string literal to concatenate.
    println!("{full_name}");
}

fn main() {
    concatination();
    format_macro();
    common_string_methods();
    user_input();
}
