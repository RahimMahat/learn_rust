/*
This file gives you a brief overview of the references and borrowing concept in the rust programming language
*/

fn immutable_reference(param: &String) {
    println!("{}", param);
}

fn mutable_reference(param: &mut String) {
    param.push_str(r"mutable_reference()");
    // return param when you are passing a mutable reference you do not need to return it
}

fn main() {
    let mut current_param = String::new();
    mutable_reference(&mut current_param); // when passing a mutable reference you need to match
    // the parameter type and the argument type as shown above
    immutable_reference(&current_param); // when only passing an immutable_reference
    // It is allowed to have multiple immutable references at the same time
    // But when it comes to the mutable reference the value must only have a single
    // mutable reference at the same time Eg.
    let mut color = String::from("Green");
    // let ref1 = &color; // This is allowed having mulitple immutable_references
    // let ref2 = &color;
    // println!("{ref1} and {ref2}");

    // let ref1 = &mut color; // But if i were to make this mutable reference you will get error.
    // let ref2 = &color; // Here when you try to set an immuable reference of the previously mutated
    // // value it won't allow, i.e only a single mutable reference at the same time
    // println!("{ref1} and {ref2}"); // this line will not compile.
    // println!("{ref2}"); // Although the compiler will allow this line to compile since it knows
    // that the mutable reference ref1 is not being utilized at the same time of ref2
    //
    // And as we've seen before an immutable reference can implement copy trait eg.
    // let coffee = String::from("Mocha");
    // let a = &coffee;
    // let b = a; // This is like let b = &coffee;
    // println!("{a} and {b}"); // this will work fine.
    // But a mutable reference does not implement copy trait so you can not have multiple owners of
    // a single mutable reference eg.
    // let a = &mut coffee;
    // let b = &mut coffee; // this will effective move the ownership & var a will not be availabe
    // down the line
    // println!("{a}"); // will throw error.

    // A dangling reference is a pointer to a memory address that has been deallocated.
    // rust does not allow the presence of a dangling reference whenever if you try to access a
    // memory addess that has been deallocated after it's deallocation the rust compiler will throw
    // error and will not compile.

    // ownership with array and tuple.
    // Composite types like arrays and tuples have ownership over their elements.
    // If a value implements the Copy trait, Rust will create a copy of it when we index into the
    // type. If a value doesn't implement the Copy trait, ownership will move from the composite
    // type to the new owner. but if you don't want to move the ownership in case of a mutable
    // value you can just define the new owner's value to have the reference to the value instead
    // of owning the value & moving the ownership.
    // Example.
    let immutable_value_array = [true, false, true];
    let first_value = immutable_value_array[0];
    println!("first value {first_value} and immutable_value_array {immutable_value_array:?}");

    let mutable_value_array = [String::from("Rust"), String::from("Python")];
    let first_language = &mutable_value_array[0]; // No move of ownership
    println!("first value {first_language} and immutable_value_array {mutable_value_array:?}");
    // The same rule apply to the tuple.
}
