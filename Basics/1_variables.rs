/*
This file gives you a brief overview of the variable concepts in the rust programming language
*/

// when you define a constant var you'll do so at the file level and that const value will be
// available to all the functions. a const can not be mutable that means you can not use mut
// keyword on a constant and more importantly the constant value has to be known to the program at
// compile time as oppose to the other vars for which the value is decided at the runtime
const TAX_RATE: f64 = 7.24;

// Type aliases is a way of creating custom type alias for your specific need
// basically assigning a var type to a new alias that you can infer while using a var of this type
type Meter = i32;

fn mutable() {
    // so if you want to make a var mutable you just add mut keyword infront of the var
    let mut gym_raps = 10;
    println!("I plan to do {gym_raps} gym raps today");

    // even if you can mutate the value now but you can not change the datatype of the var
    gym_raps = 15;
    println!("Nah i changed my mind now i'm doing {gym_raps} gym raps");
}

fn shadowing() {
    // var shadowing is basically redeclaring a var. in this case the orginal value is replace by
    // the new one
    let grams_of_protein = "100.345"; // consider a scenario where you are getting the value as str
    let grams_of_protein = 100.355; // but you want that to be a float so making the var mutable
                                    // wont' work you'll have to use var shadowing.
    let mut grams_of_protein = 100; // the equivalent int of prev var but mutable that means you can
                                    // reassign same datatype value to this var
}

fn scope() {
    // the scope of a var in rust is within the {} only so the var defined within this function
    // will go out of scope at the end of the function this area is also called as block
    // you can create an inner block by just defining {} inside the function and the inner block
    // can have access to it's own function's outer block but the vice versa is not true
    let coffe_price = 4.99;
    // this means we are creating a inner block (which will have it's own scope)
    {
        println!("The coffe price is {coffe_price}"); // this will work
        let cookie_price = 2.99; // this var goes out of scope with the end of this inner block
                                 // that means the function scope can not use this var
        let coffe_price = 20.99; // this is not var shadowing since this var is in a complete
                                 // different scope
    }
}

fn type_aliases() {
    let mile_race_length: Meter = 1700;
    let double_mile_race_length: Meter = 2 * mile_race_length;
    println!("double mile race length is {double_mile_race_length}");
}

fn compiler_directive() {
    // you can add compiler_directive to a line of code or to a fucntion or even to the entire file
    // this basically tells the compiler of the rust code how to compile the code
    // example when we have an unused var it gives you a warning so to demonstrate this concept
    // i'll show how to deal with it using compiler_directive

    #[allow(unused_variables)]
    let unused_var = "unused";
}

fn main() {
    // keep in mind that the rust variables by default are immutable
    let apples = 50;
    let oranges = 14 + 6;
    let _fruts = apples + oranges;

    println!(
        "This year my garden has {} apples and {} oranges. I can't believe i have {0} apples ",
        apples, oranges
    );

    // shows the example code for mutable var
    mutable();
    // shows the example code for shadowing
    shadowing();
    // shows the example code to explain scope
    scope();
    // while the scope of a const is not restricted by the blocks
    println!("The tax rate is {TAX_RATE}");
    // example funciton to show type aliases
    type_aliases();
    // example function to show compiler directive
    compiler_directive();
}
