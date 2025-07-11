/*
This file gives you a brief overview of the option and result enums concept in the rust programming language
*/

// The Rust prelude is a collection of named constructs that are available automatically in every program.
// In language like python we call it built in libraries or built in function, so in Rust things
// like Option, Result, derive etc are so common that they are provided as built-in.

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

fn main() {
    // all the Option enum notes are in this function.
    option();
}
