/*
This file gives you a brief overview of the generics concept in the rust programming language
*/

// A generic is a type argument. It's a placeholder for a future type.

fn identity<T>(value: T) -> T {
    // In this futnciton we're defining a future value for a type inside the angle brackets
    // defining <T> tells the function to expect a value of type T and the return value should also of the same type.
    // doing this allows up to define non-specific parameter types so the parameter annotation is
    // not bound to one single type due to which we would've had to create different function for each type.
    value
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    // Example of generic function with different Types of arguments.
    println!("{}", identity(5));
    println!("{}", identity(34.65));
    // we can also annotate the arguments during the function invocation using Turbofish Operator Eg. ::<i32>
    println!("{}", identity::<bool>(true));
    println!("{}", identity::<String>(String::from("Hello")));
    // We can combine a generic type and a concrete type in a function parameter. fn name<T>(a: T, b: i32)...
    // we can also use the same consistent generic type for multiple parameter eg. fn name<T>(a: T, b: T)...
    // we can also provide multiple generic types within the same <> eg. fn name<T, U>(a: T, b:
    // U)... in this case the argument types can be different they don't have to be consistent.
    println!("{:?}", make_tuple(true, 7.34));
    // Generics in struct. In the below example the treasure variant could be of any type T is a
    // placeholder for future type.
    #[derive(Debug)]
    struct TreasureChest<T> {
        captain: String,
        treasure: T,
    }
    let gold_chest = TreasureChest {
        captain: String::from("Jack"),
        treasure: "Gold",
    };
    let special_chest = TreasureChest {
        captain: String::from("Sparrow"),
        treasure: ["Gold", "Silver", "Diamond"],
    };
    println!(
        "Gold Chest: {:?}\nSpecial Chest: {:?}",
        gold_chest, special_chest
    );
    // Generics and impl block.
    // When you define impl block on a struct that has generic you need to define that generic on
    // the impl block as well, you can either give it a concrete type like: impl TreasureChest<String>
    // the method inside that impl block will only work where the treasure type is String.
    // or if you want the impl generic type to be scalable and accept any type of treasure value
    // you have to define the generic type on impl keyword itself and then on the struct name eg.
    impl<T> TreasureChest<T> {
        fn capital_captain(&self) -> String {
            self.captain.to_uppercase()
        }
    }

    println!("{:?}", gold_chest.capital_captain());

    // Generics in enum.
    enum Cheesesteak<T> {
        Plain,
        Topping(T),
    }

    let mashroom = Cheesesteak::Topping("Mashrooms"); // string literal
    let onion = Cheesesteak::Topping("Onion".to_string()); // heap allocated string
    // but when you want to use the other variant of the enum there you explicitly need to annotate
    // the generic type using a concrete type.
    let plain: Cheesesteak<String> = Cheesesteak::Plain;
}
