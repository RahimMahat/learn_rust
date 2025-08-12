fn main() {
    let multiplier = 5;

    // closure declaration: |parameters: type| -> return_type {code logic}
    let product = |a: f32, b: f32| -> f32 {
        print!("The product of {} and {} is ", a, b);
        a * b
    };
    // calling the closure like a function.
    println!("{}", product(3.4, 2.5));

    // in a closure you can ommit the return_type and if the logic is oneliner also the '{}'
    // can also ommit the parameter type and rust will infer the type based on first invocation.
    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(7));

    // closure that captures immutable reference.
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    let print_numbers = || println!("{:?}", numbers); // Fn captures an immutable reference so no move occured.
    print_numbers();
    println!("{:?}", numbers);

    // closure that captures a mutable reference.
    let mut nums = vec![6, 7, 8];
    let mut add_num = || nums.push(9); // FnMut: A borrow of mutable reference occured.
    add_num();
    add_num();
    println!("{:?}", nums);

    // closure with ownership.
    let first_name = String::from("John"); // heap allocated data type
    let capture_string = || first_name; // FnOnce: captures a value by ownership. i.e the value will be moved.
    // A closure with FnOnce can not be called more than once.
    // println!("{first_name}"); // can not access first_name here since move occured in the above line.
    let _owner = capture_string;

    // The 'move' keyword.
    let last_name = String::from("Wick");
    let capture_str = move || {
        // initally we were not taking ownership of the
        // last_name variable but the 'move' keyword override that behaviour and moved the value.
        println!("{}", last_name);
    };
    // println!("{}", last_name); // due to using 'move' keyword above this will now throw error.
    // Even though the value moved the capture_str is still implementing Fn trait so you can call
    // this closure multiple times.
    capture_str();
    capture_str(); // this will work.

    // The unwrap_or_else method.
    let option = Some("Shawarma");
    let food = option.unwrap_or_else(|| "Kebab"); // unwrap_or_else method expects to
    // pass a FnOnce closure in the paranthesis.
    println!("{food}");
    let option: Option<&str> = None;
    let food = option.unwrap_or_else(|| "Kebab");
    println!("{food}");
}
