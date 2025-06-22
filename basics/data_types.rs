/*
This file gives you a brief overview of the data types in the rust programming language
*/

// this is a file level compiler directive that is why it has ! after #
#![allow(unused_variables)]
// rust is a statically typed language, which means the compiler must know the types of all
// variables at compile time, the compiler can infer the types of vars based on their intial
// assignment

// A scalar type is a type that holds a single value. eg. integer, float, boolean, char etc.
// signed & unsigned integer: signed int support positive and negative values, They start with an i
// whereas unsigned int only support zero and positive values, They start with an u

// A compund type is a collection of or more values. eg. array

fn ranges() {
    // A range is a sequence/interval of consecutive values.
    let month_days = 1..31; // excluding the max limit i.e 31
    println!("{:?}", month_days);
    let month_days = 1..=31;
    println!("{:?}", month_days);

    // not covering loops extensively but demonstrating where we can use range
    for number in month_days {
        print!("{number}");
    }
    println!("");
}

fn tuples() {
    // A tuple is a collection of heterogenous data 
    let employee: (&str, i32, &str) = ("john", 28, "marketing"); // with type inference
    let employee = ("john", 28, "marketing"); // also works
    let name = employee.0; // you can access the emelemtns with . or there is another way
    let (name, age, department) = employee; // this will unpack the tuple respective to the index
    println!("{}", name);
    println!("name {}, age {}, department {}", name, age, department);
    dbg!(employee);
}

fn traits() {
    // A trait is a contract that requires that a type support one or more methods.
    // Traits establish consistency between types; methods that represent the same behaviour have
    // the same name.
    // When a type opts in to honoring a trait's requirements, we say the type implements the trait
    // Types can vary in their implementation but still implement the same trait
    // Eg. Display trait, Debug trait etc

    let seasons = ["Spring", "Summer", "Fall", "Winter"];
    // println!("{}", seasons); // this will result in compile error cuz the array does not implement
                             // display trait i.e {}
    println!("{:?}", seasons); // instead if you want to print the array you'll have to use the
                               // debug trait i.e {:?} or to pretty print the debug output you can
                               // use {:#?}
    // the debug macro - Prints and returns the value of a given expression for quick debugging
    dbg!(2 + 2); // outputs:[data_types.rs:32:5] 2 + 2 = 4 
    
}

fn arrays() {
    // An array is a fixed-sized collection of homogenous data
    let numbers: [i32; 7] = [1, 2, 3, 4, 5, 6, 7]; // just like any other vars you can prefnfine
                                                   // the type and size of the array or let the
                                                   // compiler infer the dtype & size
    let chars = ['a', 'b', 'c', 'd'];
    // let invalid = [];  // you can not intiate an empty array with no type or size inference
    // since the compiler won't know what to expect in the array. so to create an empty array you'll do
    // let valid [f32; 0] = [];

    // Reading and Writing to array
    println!(
        "The first element in numbers array is {}, and the last element in chars array is {}",
        numbers[0],
        chars[chars.len() - 1]
    );
}

fn characters() {
    // A char type represents a single unicode character, unicode is a computing standard
    // for the representation of text for most of the world's writing systems.
    let first_initial: char = 'R';
    let emoji: char = '\u{fe0f}';
}

fn logical_operators() {
    // equality & inequality operator
    println!(
        "is one euqals two {}, is one not euqal to two {}",
        1 == 2,
        1 != 2
    );
    // logical and
    println!(
        "ture AND true is {}, whereas true AND false is {}",
        true && true,
        true && false
    );
    // logical or
    println!(
        "ture OR true is {}, whereas true OR false is {}, only false OR false is {}",
        true || true,
        true || false,
        false || false
    );
}

fn boolean() {
    let is_rust: bool = true;
    println!("is the current language rust: {is_rust}");
    // boolean inversion
    println!("is the current language not rust: {}", !is_rust);
}

fn augmented_assignment() {
    let mut year = 2025; // when you want to use a var as augmented_assignment that var needs to be
                         // mutable
    year += 1;
    println!("The year + 1 is {year}");

    year -= 5;
    println!("The year + 5 is {year}");
}

fn arithmetics() {
    let addition = 5 + 3;
    let substraction = 5 - 3;
    let multiplication = 5 * 3;
    let floor_division = 5 / 3; // when you divide int / int rust performs floor division and
                                // returns int
    let decimal_division = 5.0 / 3.0;
    println!("Addition {addition}, Substraction {substraction}, Multiplication {multiplication}, Floor division {floor_division}, Decimal division {decimal_division}");
}

fn type_casting() {
    // type casting is basicallly casting a data type into a different data type that respects the
    // constraints of the new data type
    let miles_away: i32 = 50;
    let miles_away_i8 = miles_away as u8;

    let miles_away: f64 = 100.834845; // var shadowing
    let miles_away_int = miles_away as i32;
    println!("miles away int {miles_away_int}");
}

fn floats() {
    let pi: f64 = 3.1415926535897932384; // f64 is the default float value, float don't have the
                                         // concept of signed or unsigned float
    println!("the current value of pi is {pi}");

    // A format specifier customizes the printed representation of the interpolated value
    println!("the formated value of pi is {pi:.2}"); // this is one way of doing it
    println!("the formated value of pi is {:.4}", pi);
}

fn methods() {
    // A method is a function that lives on a value. it's an action we can ask the value to execute
    // eg. value.method()
    let value: i32 = -3373;
    println!("the absolute value is {}", value.abs());
    println!("the square of value is {}", value.pow(2));

    let empty_spaces: &str = "   empty spaces            ";
    println!("{}", empty_spaces.trim());
}

fn strings() {
    // string is collection of characters in a sequence
    let string: &str = "Hello, world!";
    println!("\t{string}"); // just like other languages you can make use of escape chars
}

fn usize_isize() {
    // usize & isize are basicallly the unsigned and signed var but the size of the var is infered
    // based on your computer architecture so if you have 32bit system it'll be i32 or u32 and same
    // goes for 64bit systems.
    let days: usize = 32342;
    let temp: isize = -22;
}

fn integers() {
    let eight_bit: i8 = -102;
    let eight_bit_unsigned: u8 = 230;

    let thirty_two_bit = -2147483648; // i32 is the default value when you assign a int var
    let thirty_two_bit_unsigned: u32 = 4294967295; // since an unsigned int can not store a
                                                   // negative value you get twice the space for
                                                   // positive values compared to signed int
}

fn main() {
    integers();
    usize_isize();
    strings();
    methods();
    floats();
    type_casting();
    arithmetics();
    augmented_assignment();
    boolean();
    logical_operators();
    characters();
    arrays();
    traits();
    tuples();
    ranges();
}
