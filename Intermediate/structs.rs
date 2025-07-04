/*
This file gives you a brief overview of the structs concept in the rust programming language
*/

// A strcut (structure) is a container for related pieces of data.
// Rust has 3 kinds of structs:
// i. Named Field structs
// ii. Tuple-like structs
// iii. Unit-like structs

// if you were to define the struct outside at the file level every function in the file can access it.
// and can be used as a return type for the other functions eg. fn make_coffee(name: String, price: f64) -> Coffee {function logic}
#[derive(Debug)]
struct Car {
    name: String,
    model: i32,
    price: f64,
}

#[derive(Debug)]
struct ProgrammingLanguages {
    name: String,
    founder: String,
    release_year: u32,
}
// impl stands for implementataion in which we can define the methods for a struct
// you can have multiple impl blocks with the same name if they represent they same struct if you
// want to visually differentiate some part of your logical code to a different impl block you can
// do so like: impl ProgrammingLanguages {code...} Rust while compiling will combine thiese impl
// blocks together.
impl ProgrammingLanguages {
    // self is of type ProgrammingLanguages like self: ProgrammingLanguages or it's also written as self: Self

    // Associated functions are functions that are attached to a type Eg. String::from() 'from' is an associated function that lives on a type which is String.
    // A constructor is a function that retrns a new instance of a type. Eg. String::new()
    // How rust differentiates between methods and associated function in an impl block is when you
    // don't have the 'self' parameter it will consider that to be associated function and if you
    // do have the 'self' parameter then it will be treated as a method.
    fn new(name: String, founder: String, release_year: u32) -> ProgrammingLanguages {
        Self {
            name,
            founder,
            release_year,
        }
    }

    // Imutable struct value (self parameter takes ownership)
    fn _display_language_info(self: ProgrammingLanguages) {
        println!(
            "The Programming Language {} was developed by {} in {}",
            self.name, self.founder, self.release_year
        );
    }
    // Mutable struct value (self parameter takes ownership, has permission to mutate)
    fn _double_release_year(mut self: Self) {
        // same thing as annotating ProgrammingLanguages
        self.release_year = self.release_year * 2;
        println!("{:?}", self);
    }
    // Immutable reference to the struct instance (no ownership moved)
    fn _ref_display_language_info(self: &Self) {
        println!(
            "The Programming Language {} was developed by {} in {}",
            self.name, self.founder, self.release_year
        );
    }
    // Mutable reference to the struct instance (no ownership moved, have permission to mutate)
    fn _mut_ref_double_release_year(self: &mut ProgrammingLanguages) {
        // same thing as annotating ProgrammingLanguages
        self.release_year = self.release_year * 2;
        println!("{:?}", self);
    }
    // Shorthand annotation for all 4 types are self, mut self, &self, &mut self

    // A method can accept parameter just like any function can only condition being the self
    // parameter is mandatory for every method.
    fn is_older_than(&self, other: &ProgrammingLanguages) -> bool {
        self.release_year < other.release_year
    }
    // you can call methods within a mehtod
    fn display_language_info_since_release_year(self: &Self) {
        println!(
            "The Programming Language {} was developed by {}, it was release {} years ago",
            self.name,
            self.founder,
            self.years_since_release()
        );
    }
    fn years_since_release(&self) -> u32 {
        2025 - self.release_year
    }
}

fn make_car(name: String, model: i32, price: f64) -> Car {
    // If your parameter and the field name are the same you can use the shorthand assignment and
    // just type the param name once istead of doing name: name etc.
    Car { name, model, price }
}

fn drive_car(car: Car) {
    // structs allows mutablality you can just as easily do (mut car: Car) in the function parameter section.
    // instead of passing the Car instance you can also pass in the reference to the instance like (car: &Car)
    // or (car: &mut Car) for mutable reference in this case the ownership is not moved.
    println!("Driving my fast {}", car.name);
}

fn main() {
    // For a struct name we use PascalCase but var name is in snake_case that is the community convention.
    // A struct is somewhat similar to a class in an OOP language, it's a blueprint for an object.
    struct Coffee {
        name: String,
        price: f64,
        is_hot: bool,
    }
    // An instance is the concrete value made from a type.
    // for a named field struct you can define the values in whatever order the values are tied to
    // the name and not order of the struct field.
    let mocha = Coffee {
        price: 5.99,
        name: String::from("Mocha"),
        is_hot: true,
    };
    // accessing the value in struct: struct.field
    println!(
        "My {}, this morning cost {}. It is {} that it was hot.",
        mocha.name, mocha.price, mocha.is_hot
    );
    // you can have a mutable copy or immutable copy of a struct.
    let mut beverage = Coffee {
        name: String::from("Cappucino"),
        price: 7.99,
        is_hot: false,
    };

    beverage.name = String::from("Caramel Macchiato");
    beverage.is_hot = true;
    println!(
        "My {}, this morning cost {}. It is {} that it was hot.",
        beverage.name, beverage.price, beverage.is_hot
    );

    let name = String::from("Mercedes");
    let model = 2025;
    let price = 34798732.43;
    let my_car = make_car(name, model, price);
    println!(
        "my car info: name {} model {} price {}",
        my_car.name, my_car.model, my_car.price
    );
    // Rust allows us to copy the fields of a struct instance into a different instance eg.
    // They are 2 independant instances. for whichever field doesn't implement copy trait in that
    // case the ownership will be moved.
    let bmw = Car {
        name: String::from("BMW"),
        ..my_car
    };
    // what will happen in the above block is rust compiler will see that we have denied the name
    // field so it'll stay as we provided, but the rest of the fields will be copied from the
    // my_car instance of Car to bmw instance. ..<instance_name> var always has to be put last.
    println!(
        "bmw info: name {} model {} price {}",
        bmw.name, bmw.model, bmw.price
    );

    // Passing struct into a function.
    drive_car(my_car);
    // println!("{}", my_car.name); // now this line will not be able to compile becasue in the above
    // line when we pass the my_car instance that moves the ownership to drive_car function and
    // since we are not returning anything my_car is also cleared from memory in that function itself.

    // A struct by default does neither implements display trait nor debug trait
    // but rather Rust allows us to derive a debug trait to represent the struct
    // To do this you have to writea compiler directive on the struct definition #[derive(<trait to be derived>)]
    // once you derive the debug trait now you can use it to represent the struct instances.
    println!("{:?}", bmw);

    // using methods on the struct instances
    let rust = ProgrammingLanguages {
        name: String::from("Rust"),
        founder: String::from("Graydon Hoare"),
        release_year: 2006,
    };
    // rust.display_language_info(); // these 2 methods move ownership since it take the values
    // rust.double_release_year();
    // rust.ref_display_language_info(); // but these 2 don't move ownership since it takes reference
    // rust.mut_ref_double_release_year(); // for this to work you need to define the struct instance as mutable: let mut rust = ProgrammingLanguages {}

    // passing parameters to a method
    let c = ProgrammingLanguages {
        name: String::from("C"),
        founder: String::from("Dennis Ritchie"),
        release_year: 1972,
    };
    if c.is_older_than(&rust) {
        println!("{} is older than {}", c.name, rust.name);
    } else {
        println!("{} is not older than {}", c.name, rust.name);
    }
    // nested method example
    c.display_language_info_since_release_year();

    // callling an associated function
    let _python = ProgrammingLanguages::new(
        String::from("Python"),
        String::from("Guido van Rossum"),
        1991,
    );
}
