/*
This file gives you a brief overview of the vectors concept in the rust programming language
*/

fn main() {
    // Vectors are re-sizable arrays, like slices their size is not know at compile time.
    // A vector is specially usefull when the elements in the array are supposed to grow or shrink
    // as needed but if you have a fix number of elements arrays do the job.
    // Vecots follows the same rules of ownership and borrowing as we've seen with other.

    // Initializing a vector.
    let empty_vec: Vec<i32> = Vec::new(); // we can annotate the type 
    let str_vec = Vec::<&str>::new(); // or use a turbofish operator
    println!("empty vec: {empty_vec:?} str vec: {str_vec:?}");
    // if we know the starter value that we'd want to populate the vector with we can use the 'vec!' macro.
    let mut num = vec![2, 6, 8]; // when we have the initial values we can but we don't need to
    // annotate the type while creating the vec Rust will infer the data type from the values in the vec.
    println!("numbers {num:?}");

    // adding or removing elements from vector.
    // to add a single element in a vec we can use .push method
    num.push(10);
    num.push(14);
    println!("numbers after 2 push {num:?}");
    // to add an element in a given index we can use .insert method
    num.insert(1, 4);
    println!("inserting number 4 at index 1 using .insert {num:?}");
    // to remove the last element and return it we can use .pop method
    let last_element = num.pop();
    println!(
        "numbers vector after .pop {num:?} and the returned element is {}",
        last_element.unwrap()
    );
    // and to remove an element from the given index we can use .remove method
    num.remove(0);
    println!("numbers vector after removing 0th index element using .remove {num:?}");

    // Reading vector elements
    let abc = String::from("ABC");
    let def = String::from("DEF");
    let ghi = String::from("GHI");
    let str_vec = vec![abc, def, ghi];
    // we can use the same [] method to access & read an element at a given index but the move of
    // ownership will depend on the type of data that vector stores, for example if the vector has
    // integers and they implement Copy trait so when you take out an element the ownership will
    // not move but the element will be copied to the new variable, but if you do the same on
    // String vectors and as we know String does not implement Copy trait in this case the
    // ownership will be moved to the new variable and so it will not be allowed by the compiler
    // and to resolve this just like we did before we can either clone the element or use the reference.
    let num_value = num[2]; // this will create a copy without moving the ownership
    let str_value = &str_vec[2]; // since we're only using reference of the value move will not occur.
    println!("at 3rd position num value {num_value} and reference to the str value {str_value}");
    let slice_num = &num[..2];
    let slice_str = &str_vec[1..];
    println!("slice of num vector {slice_num:?} and slice of str vec {slice_str:?}");
    // The .get method exacts a vector element by index position. It returns an Option enum to
    // avoid the runtime panic if the index doesn't exist. .get method always returns the reference
    // to the value within the Option variant.
    let option_num = num.get(4);
    match option_num {
        Some(some_num) => println!("the num extracted from .get is {some_num}"),
        None => println!("Index out of bound"),
    }
    // Writing vector elements
    num[0] = 2;
    println!("Overwriting 0th position element {num:?} ");

    // The vector capacity is the maximum number of elements that the vector can contain.
    // to demonstrate the capacity we'll use another constructor that the Vec type has which allows
    // us to set the capacity while Initializing the Vec
    let mut vec_with_capacity: Vec<&str> = Vec::with_capacity(3);
    println!(
        "Before = Length: {}, Capacity: {}",
        vec_with_capacity.len(),
        vec_with_capacity.capacity()
    );
    vec_with_capacity.push("Summer");
    vec_with_capacity.push("Winter");
    vec_with_capacity.push("Rainy");
    println!(
        "After = Length: {}, Capacity: {}",
        vec_with_capacity.len(),
        vec_with_capacity.capacity()
    );
    vec_with_capacity.push("Autumn"); // now since a Vector is still a growable type it'll allow
    // this even if we set the capacity to 3 but behind the scenne it needs to look for a larger
    // memory location to store the data and increase the capacity of the vector in runtime.
    println!(
        "After exceeding capacity = Length: {}, Capacity: {}",
        vec_with_capacity.len(),
        vec_with_capacity.capacity()
    );
}
