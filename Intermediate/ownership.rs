/*
This file gives you a brief overview of the ownership concept in the rust programming language
*/

// Ownership is a set of rules that the compiler checks for to ensure the progrrm will be free of
// memory errors.
// Memeory referes to the area of your computer that is resposible for storing the information your
// program use.
// It's ideal to free memory when it is no longer in use.
// Various programming languages implement different ways to manage memeory languages like c/c++
// it's the developers resposiblity to allocate or deallocate the memory while the languages like
// python, java, ruby or go implement garbage collection which slows down the program.
// Rust introduces a new paradigm: Ownership
// The Rust compiler does not compile the program if an ownership rule is violated.
// Best of all worlds: the speed of a language like c but with less room for error.
//
// What is Ownership?
// The owner is who/what is responsible for cleaning up a piece of data when it is no longer in use
// Every value in a Rust program has one owner.
// The owner can change over the course of the program, but there is only 1 owner for a value at a
// time.
// The owner is usuallly a name. Eg. variable or parameter can be an owner
// Ownership also extends to composite types that own their elements, A tuple and array own their
// values.
//
// The Stack & The Heap:
// The stack and the heap are two different parts/regions of the compter's memeory.
// The stack and heap read and write data in different ways that offer advantages and disadvantages
// A stack stores values in the sequential order it receives them.
// A stack is last in first out (LIFO). The last item added is the first one removed.
// The technical terminology for adding data is pushing onto the stack.
// The technical terminology for removing data is popping off the stack.
// All stack data has a fixed, consistant size that is known at compile time.
// Data types like integres, foating-points, booleans, characters, and arrays have a fixed size,
// Rust stores them on the stack at runtime.
// The pieces of data on the stack will not grow or shrink in size as the program runs.
// The heap is a large area of storage space, Thank of it like a warehous.
// The heap is for data whose size is not known at compile time (user input, a file's contents, etc)
// When the Rust program needs dynamic space, it requests it from the heap. A program called the
// memory allocator finds an empty spot that is large enough to store the data.
// The memory allocator returns a reference, which is an address.
// The reference points to the memory address of the data.
// Think of a parking lot giving you a reference (spot "H25") when they park your car.
// We can store a reference in a variable in a Rust program. Refereces have a fixed size, so Rust
// stores them on the stack.
// Allocating on the heap is slower than pushing to the stack. The memory allocator has to spend
// time searching for an open spot large enough to fit the data.
// Accessing data is faster on the stack than the heap as well. With a heap, the program has to
// follow the pointer to find the memory address.
// A stack stores the data in sequence, so there is less "jumping around" from point to point.
// The purpose of ownership is to assign resposiblity for deallocating memory (primary heap memory)
// Ownership is a compiler feature for reducing duplicate heap data and cleaning up heap data that
// is no longer needed.

fn moves_and_ownership() {
    // A move is the transfer of ownership from one owner to another.
    // Unlike a stack allocated dtype a heap allocated dtype does not implement copy trait
    // So in the below example when you reassign the person variable to the genius variable that
    // effectively transfers the ownership from person to the genius variable and now instead of
    // person the genius variable is resposible for cleaning up the heap entry for that data.
    // And since we've transfered the ownership to genius that means person variable goes out of
    // scope and can no longer be used once the ownership transfer happens.
    let person = String::from("John");
    println!("Random person {person}"); // This will work
    let genius = person;
    // println!("Random person {person}"); // But this will throw a compile time error 
    println!("Now the random person is {genius}");
    // you can manually deallocate a heap stored memory using drop function.
    drop(genius);
}

fn string_type() {
    // the string type we've been using like let food: &str = "pasta"; The value is called string
    // literal and it is neither stored in heap nor stack since the value is already know to the
    // program rust includes this value in the executable binary. 
    // In this method we'll be exploring the String type. We use String when we don't know the size
    // of the value at compile time and so it is stored in the heap.
    let _text = String::new();
    let _candy = String::from("KitKat"); // you can also create String with a given value using from
    // you can concatinate the String using push_str method
    let mut name = String::from("Rahim");
    name.push_str(" Mahat");
    println!("{name}");
}

fn copy_trait() {
    // manay of rust's scaler data types implement copy trait eg. int, float, bool etc
    // Eg 
    let time = 2025;
    let year = time; // time beign a i32 here is being copied by year var
    // The time and year are 2 different owners of 2 values and are resposible for their data
    println!("The time is {time} and the year is {year}");
}

fn main(){
    copy_trait();
    string_type();
    moves_and_ownership();
}
