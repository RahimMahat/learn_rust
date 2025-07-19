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

#[allow(dead_code)] // this compiler directive is only applied to the foloowing function.
fn ownership_and_return() {
    // A return value allows us to persist a value even though a function is concluding.
    // if there is no new owner of the return value from where the function is called the memory
    // will be deallocated.
}

fn mutable_parameters(mut param: String) {
    // just like variables the function parameters are immutable by default.
    // to make a parameter mutable you have to define the parameter with the mut keyword
    // since with the heap values when you pass that as a param the ownership is transfer so even
    // if you'd defined this value as mutable in main method it would still not work here if you
    // didn't had the mut keyword infront of this function's parameter.
    // for a parameter to be mutable the variable does not need to be mutable.
    param.push_str(". param added in a function where the parameter is mutable");
    println!("{param}");
}

fn funcitons_and_ownership(stack_value: i32, heap_value: String) {
    // When you pass a stack parameter the copy of the param value is passed cuz stack values
    // implement copy trait. However when a heap value is passed it effectively transfers the
    // ownership of that value.
    println!("stack value in funcitons_and_ownership {stack_value}");
    println!("heap value in funcitons_and_ownership {heap_value}");
}

fn referece_and_borrowing() {
    // Borrowing means using something without taking ownership of it.
    // creating a stack reference is relatively less expensive than creating a heap reference.
    // Refereces must never outlive their referent.
    let my_stack_value = 7;
    // As we learnt the stack types implement copy trait just like that the stack references alos
    // implement copy trait since they are references there will not be transfer of ownership.
    let my_stack_reference = &my_stack_value; // this does not store the value of the
    // my_stack_value which will transfer the ownership but rather only stores the address of that
    // variable. you can simply type annotate this like below.
    let my_heap_value: String = String::from("Audi");
    let my_heap_reference: &String = &my_heap_value;
    // To dereference means to access  the data at the memory address that the referece points to.
    println!(
        "The dereferced value of vars is {} & {}",
        *my_stack_reference,
        my_heap_reference // the reference value implement the display trait
                          // so even if you were to just give the reference var without * operator it will still
                          // print the value of the reference and not the memory address.
    );
    // basically '*' operator means to take the reference of a value follow it and get the actual
    // value of that address.
}

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
    // Instead of implementing copy trait and transfering ownership to another variable you can
    // implement clone trait and with this the ownership will not be transfer but the genius
    // variable will be responsible for it's own cloned object and person for it's own.
    // and when you clone instead of copy you can access the person var even after the cloned var
    let another_person = genius.clone();
    println!("another_person is {another_person} and genius is {genius}");
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

fn main() {
    copy_trait();
    string_type();
    moves_and_ownership();
    referece_and_borrowing();
    // Ownership and function parameters
    let stack_value = 7;
    let heap_value = String::from("Apple");
    funcitons_and_ownership(stack_value, heap_value); // for the stack_value a copy of the value is passed. while for the heap_value the value itself is passed.
    println!("stack value in main {stack_value}");
    // println!("heap value in main {heap_value}"); // This will throw a compile time error. since the
    // value is borrowed here after the move (transfer of ownership). If you want this to work you
    // can implement the clone trait and pass the clone as parameter
    let param_from_main = String::from("param from main");
    mutable_parameters(param_from_main);
}
