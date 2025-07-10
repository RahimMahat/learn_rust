/*
This file gives you a brief overview of the option and result enums concept in the rust programming language
*/

// The Option enum models a scenario where a type could be a valid value or nothing at all.
// Option::None represents an absent value. Option::Some(T) represents present value.

fn main() {
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
}
