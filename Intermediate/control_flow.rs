/*
This file gives you a brief overview of the control flow in the rust programming language
*/

fn match_expr() {
    // The match statement can react to all possible variants of a value
    // A pattern or arm is one possible option to compare the match value against.
    let season = "";

    match season {
        "summer" => {
            println!("Schools out!!!");
        }
        "winter" => {
            println!("Brr, so cold!");
        }
        _ => {
            println!("It's raining, no wait it's hot, nope it's cold???");
        }
    }

    // In Rust you can compare multiple values in a single arm
    let number = 8;
    // match number {
    //     2 | 4 | 6 | 8 => println!("{number} is even"),
    //     1 | 3 | 5 | 7 => println!("{number} is odd"),
    //     _ => println!("Unknown for now"),
    // }

    match number {
        value if value % 2 == 0 => println!("The number {value} is even"),
        value if value % 2 != 0 => println!("The number {value} is odd"),
        _ => unreachable!(), // in case of even-odd the catch-all arm will never be invoked so we
                             // can just use a macro called unreachable and that should take care of it.
    }
}

fn even_or_odd(number: i32) {
    // Rust does not have a dedicated ternary operator, However you can achieve the same
    // functionality using the if expression
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number {number} is {result}");
}

fn main() {
    // The purpose fo this function is to show the if else conditional and just like any other
    // language you can also use 'else if' with if-else statement if you require it.
    even_or_odd(24);
    match_expr();
}
