/*
This file gives you a brief overview of the enum concepts in the rust programming language
*/

// An enum is a type that represents a set of possible values. Each possible value is called a variant.

use std::os::fd::OwnedFd;

// We declare the enum name and variant name in PascaCase and since the element in the enum block
// are the variants we don't need to wrap them around quotes like a string. Enum like struct don't
// implement display or debug trait but we can derive the debug trait just like struct.
#[derive(Debug)]
enum CardSuit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

// #[derive(Debug)]
// struct Credentials {
//     username: String,
//     password: String,
// }
#[derive(Debug)]
enum PaymentMethodType {
    // when you want some associated data along with the enum variant you add
    // (data_types_of_associated_values) to the variant. All variants don't have to have the same
    // associated values it can differ as you need.
    CreditCard(String),
    DebitCard(String),
    // PayPal(String, String),
    // A struct variant stores associated data in fields rather than by position. Each piece of data has an associated name.
    // PayPal(Credentials), // you can define the struct associated value like this but rust also
    // allows a simpler and easier syntax for immitating this behaviour, given below
    PayPal { username: String, password: String },
}

// Nesting enums in enum is allowed
#[derive(Debug)]
enum Meat {
    Chicken,
    Beef,
}
#[derive(Debug)]
enum RestaurantItem {
    Burrito(Meat),
    Biryani(Meat),
    SaladBowl,
}

// using enums with match case statement
enum OperatingSystem {
    Linux { distro: String },
    Windows(u32),
    MacOS,
}

// just like struct we can implement methods and associated functions for enums using impl keyword
impl OperatingSystem {
    fn years_since_release(&self) -> u32 {
        match self {
            OperatingSystem::Linux { distro } => {
                println!("The current linux distro is {distro}");
                34
            }
            OperatingSystem::Windows(version) => {
                println!("This is Windows {version}");
                39
            }
            OperatingSystem::MacOS => 24,
        }
    }
}

enum Milk {
    LowFat(i32),
    Whole,
    NoDairy { kind: String },
}

fn main() {
    // same mutability, ownership or borrowing principles apply to these enums as well
    let first_card: CardSuit = CardSuit::Spades;
    let mut second_card = CardSuit::Hearts;
    second_card = CardSuit::Diamonds;

    let visa = PaymentMethodType::CreditCard(String::from("0034-578"));
    let master_card = PaymentMethodType::DebitCard(String::from("1234-987"));
    println!("master card: {master_card:?}, visa: {visa:?}");

    // Demonstrating multiple associated value variant.
    //    let mut payment_method = PaymentMethodType::CreditCard(String::from("1234-234"));
    //    payment_method =
    //        PaymentMethodType::PayPal(String::from("john@email.com"), String::from("password"));
    //    println!("payment method {payment_method:?}");

    // Demonstrating struct as an associated value for a variant.
    // With the alternate method shown above in the enum you don't need to define a separate struct
    // var the values can directly be passed to the variant.
    // let paypal_creds = Credentials {
    //     username: String::from("username@email.com"),
    //     password: String::from("password"),
    // };
    // let paypal = PaymentMethodType::PayPal(paypal_creds);
    let paypal = PaymentMethodType::PayPal {
        username: String::from("username@email.com"),
        password: String::from("password"),
    };
    println!("{paypal:?}");

    // Demonstrating nested enums.
    let salad_bowl = RestaurantItem::SaladBowl;
    let lunch = RestaurantItem::Burrito(Meat::Chicken);
    let dinner = RestaurantItem::Biryani(Meat::Beef);
    println!("Breakfast was {salad_bowl:?}, Lunch was {lunch:?} and dinner was {dinner:?}");

    // Demonstrating match case with enums having different types of associated value/data. and impl
    let my_system = OperatingSystem::Linux {
        distro: String::from("Arch"),
    };
    let age = my_system.years_since_release();
    println!("My system is {age} years old");
    let other_system = OperatingSystem::Windows(11);
    other_system.years_since_release();

    // The if let construct combines an if statement with a variable declaration.
    let my_beverage = Milk::LowFat(2); // assume this value is coming dynamically

    if let Milk::LowFat(percent) = my_beverage {
        println!("Your beverage is {percent}% milk");
    }
}
