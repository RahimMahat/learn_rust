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
}
