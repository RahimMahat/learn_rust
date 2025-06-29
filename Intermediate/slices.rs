/*
This file gives you a brief overview of the slices concept in the rust programming language
*/

// A collection type is one that can hold mutiple values, Arrays, tuples and strings are colleciton types.
// A slice is a reference ot a portion/sequence of a collection type. It's a subcategory or referece.
// A string slice is a reference to a sequence of characters from a string.
// An array slice is a reference to a sequence of elements from an array.
// As a reference, a slice does not take ownership of the collection.

fn array_slice(array: &[i32]) {
    // with string a slice may indicate the size of a byte but with array it indicates the index of the array.
    println!("length of array: {}", array.len());
    let array_slice = &array[..3];
    println!("array slice: {array_slice:?}");

    // Rust does permit mutable slices of array
    let mut another_array = [1, 2, 3, 4, 5, 6, 7];
    let another_array_slice = &mut another_array[2..4];
    println!("mutable slice of the array: {another_array_slice:?}");
    another_array_slice[0] = 100;
    println!("after mutating the array slice is: {another_array_slice:?}");
    // because of the change in mutable array slice the original array will also be affected
    println!("complete array: {another_array:?}");
}

fn string_slice(andrew: &str) {
    // String slice
    // Rust does not permit mutable slices of string.
    // Heap allocated string.
    let iron_man = String::from("Iron Man");
    let string_slice = &iron_man[0..4]; // inside the [] you define a byte range not a character range
    // so the range 0..4 says start from 0th byte of the string untill 4th byte
    println!("{string_slice}");

    // string literal: (ref str)
    let spider_man = "Spider Man";
    let ref_str_slice = &spider_man[7..10];
    println!("{ref_str_slice}");
    println!("spider man actor is {andrew}");

    // The length of a string slice refers to a count of it's bytes, not it's characters.
    let universe = "Marvel";
    println!("len(universe): {}", universe.len());
    println!("len(string_slice): {}", string_slice.len());
    let universe_emoji = "🌌"; // this emoji occupies 4bytes of memory
    println!("len(universe_emoji): {}", universe_emoji.len()); // so you'll get the length as 4.

    // Syntactic shortcuts
    let actor = String::from("Robert Downey Jr");
    let first_name = &actor[..6];
    let last_name = &actor[7..];
    println!("Iron Man: {first_name} {last_name}");
}

fn main() {
    let andrew = "Andrew Garfield";
    // passing a string slice as a parameter.
    string_slice(&andrew[..]);

    // passing an array slice as a parameter
    let values = [4, 7, 8, 16, 23, 42, 49];
    array_slice(&values[..]);
}
