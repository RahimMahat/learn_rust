/*
This file gives you a brief overview of the functions in the rust programming language
*/

fn cube(number: i32) -> i32 {
    // when you want to implicitly return the last evaluated line you don't want to terminate it
    // with ';' or else rust will treat it like any other statement and not implicit return
    number.pow(3)
}

fn square(number: i32) -> i32 {
    return number * number;
}
fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas");
}

fn open_store(neighbourhood: &str) {
    println!("Opening my pizza store in {neighbourhood}");
}

fn alphabet(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}

fn main() {
    // A function is a sequnce of steps to be executed in order
    // A parameter is a name for an expected input to a function
    // An argument is the concrete value passed in for a prameter when the function is invoked

    open_store("Brooklyn");
    bake_pizza(20, "pepperoni");

    // A return value is the output of a function
    let square = square(5);
    println!("The square of 5 is {square}");
    // implicit return: rust will return the last line of a function that it evaluates
    let cube = cube(3);
    println!("The cube of 3 is {cube}");
    // A unit is an empty tuple, a tuple without values. This is the default return value of a
    // function if no return value is specified either implicitly or explicitly

    // You can define a nested block within a function which will have it's own scope
    let multiplier = 3;

    let calculation = {
        let value = 3 + 4; // This block has it's own scope and can access the vars defined in
        // it's outer scope but the outer scope can not access the inner block vars unless returned
        value * multiplier
    };

    println!("calculation is {calculation}");

    // define an alphabet function and check if the text passed contains the letter 'a', 'z' and
    // retrn respective boolean;
    println!("{:?}", alphabet("zoology"));
}
