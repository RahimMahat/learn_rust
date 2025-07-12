/*
This file gives you a brief overview of the option and result enums concept in the rust programming language
*/

// The Rust prelude is a collection of named constructs that are available automatically in every program.
// so in Rust things like Option, Result etc are so common that they are provided as built-in.

use std::{num::ParseIntError, vec};

fn option() {
    // The Option enum models a scenario where a type could be a valid value or nothing at all.
    // Option::None represents an absent value. Option::Some(T) represents present value.

    fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
        // when you are returning an Option enum from a function you need to specify the generic type
        if item_is_in_system && item_is_in_stock {
            Option::Some(true)
        } else if item_is_in_system {
            Option::Some(false)
        } else {
            Option::None
        }
    }

    let _a: Option<i8> = Option::Some(7);
    let _b: Option<&str> = Option::None; // when you are defining a None Option you do have to
    // pre-define the generic type of the Option.

    let music_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = music_instruments.get(2); // .get method returns an option,
    // so you'll either get the value present at the index and if index is out of bounds then None Option.
    println!("{bass:?}");
    let invalid = music_instruments.get(5); // since index 5 doesn't exist
    // in the music_instruments array .get method will return a None option but if we were to
    // utilize the other syntax we would've got an error
    println!("{invalid:?}");

    // An Option type is a wrapper around a core value.
    // The unwrap method attempts to extract the associated data out of the Some variant.
    let valid_instrument = bass.unwrap();
    println!("Unwrapped value for Some variant: {valid_instrument}");
    // The problem with unwrap is when we're dealing with the None variant the compiler will panic
    // and we'll run into a runtime error. There is a similar method like unwrap called expect,
    // expect allows us to customize the error message, instead of the runtime error/panic msg.
    // invalid.expect("Unable to retrieve element.");
    // unwrap & expect both are optimistic in it's approach where they assume Some option to be returned.
    // They are great for developers as they provide quick values to debug or develop.
    // But
    // There is a more common approach that you'll find used more then others is using the match keyword
    match bass {
        Option::Some(instrument) => println!("Playing the {instrument}"), // for bass this will run.
        Option::None => println!("Defaulting to a vocalist"),
    };
    match invalid {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Defaulting to a vocalist"), // for the invalid value this will run.
    };

    // Returning an Option enum from a function.
    let availability = is_item_in_stock(true, false);
    // println!("{availability:?}");
    // match availability {
    //     Option::Some(true) => println!("Yes, the item exist in the system"),
    //     Option::Some(false) => println!("No, the item is not in stock"),
    //     Option::None => println!("Your item does not exist in our system"),
    // }
    // since the Option enum is provided to us as Rust prelude we can ommit the Option prefix from
    // above match construct. like this and it'll work the same.
    match availability {
        Some(true) => println!("Yes, the item exist in the system"),
        Some(false) => println!("No, the item is not in stock"),
        None => println!("Your item does not exist in our system"),
    }
    // The unwrap_or method. This works similar to the unwrap method on Option enum but
    // additionally allows you to set a default fallback value in case you get None.
    let present_value: Option<i32> = Some(7);
    let absent_value: Option<i32> = None;
    println!(
        "some value: {}, none value (defaulted): {}",
        present_value.unwrap(),
        absent_value.unwrap_or(0)
    );

    // Building Option from scratch.
    #[derive(Debug, Clone, Copy)]
    enum MyOption {
        Some(i32),
        None,
    }

    impl MyOption {
        fn unwrap(self) -> i32 {
            match self {
                MyOption::Some(value) => value,
                MyOption::None => panic!("Called a None value"),
            }
        }
        fn unwrap_or(self, fallback_value: i32) -> i32 {
            match self {
                MyOption::Some(value) => value,
                MyOption::None => fallback_value,
            }
        }
    }

    let some_myoption = MyOption::Some(49);
    println!("some option: {}", some_myoption.unwrap());
    let none_myoption = MyOption::None;
    // println!("none option: {}", none_myoption.unwrap()); // this line will panic
    println!("none option (defaulted): {}", none_myoption.unwrap_or(50)); // this will not panic
}

fn result() {
    // The result enum models an evaluation that can produce either a success or an error.
    // The Ok variant indicates piece of data of generic type T.
    // The Err variant indicates an error. It stores an associated piece of data of generic type E.
    let ok: Result<i8, &str> = Result::Ok(4);
    let err: Result<i32, &str> = Result::Err("Something went wrong");
    println!("Ok: {ok:?}, Error: {err:?}");

    let text = "23";
    let text_as_number: Result<i16, ParseIntError> = text.parse::<i16>(); // The parse method
    // returns a result enum so we need to provide the type which we are trying to parse from a
    // value using turbofish operator.
    println!("text_as_number ok: {text_as_number:?}");
    let text = "Not a Number";
    let text_as_number = text.parse::<i16>();
    println!("text_as_number err: {text_as_number:?}");

    // Let's define a function that will return a Result enum.
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Can not divide by zero".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }
    let divide_ok = divide(10.0, 2.0);
    match divide_ok {
        Ok(calculation) => println!("Result: {}", calculation),
        Err(message) => println!("Error {}", message),
    }
    let divide_err = divide(10.0, 0.0);
    println!("result err: {}", divide_err.unwrap_or(0.0)); // just like Option, Result also has unwrap, unwrap_or etc methods

    fn operation(great_success: bool) -> Result<&'static str, &'static str> {
        if great_success {
            Ok("Success")
        } else {
            Err("Error")
        }
    }

    let my_result = operation(true);
    let _content = match my_result {
        Ok(msg) => msg,
        Err(err) => err,
    };
    println!("my result: {}", my_result.unwrap()); // if we had a datatype that does not implement
    // Copy trait like heap allocated 'String' we would not be able to access the unwrap method on my_result
    // variable because the ownership would've moved to the content variable.
}

fn if_let_while_let() {
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    if let Some(sauce) = sauces.pop() {
        // the pop method returns Option so we're
        // checking if we're getting Some option and if we do we'll print it out
        println!("The next sauce is {sauce}");
    }
    // we can either keep repeating the if let constrct again and again untll there is None. But
    // this is not optimal, and for this we use while let which will run the while loop untill
    // there is Some option and as soon as it gets None option it'll terminate
    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}");
    }
}

fn main() {
    // all the Option enum notes are in this function.
    option();
    // all the Result enum notes are in this function.
    result();
    // if let and while let
    if_let_while_let();
}
