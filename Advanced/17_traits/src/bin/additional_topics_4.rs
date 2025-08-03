struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

// Immplementing PartialEq trait on a struct. you can also derive PartialEq trait directly on the
// struct using compiler directive the difference being when you use #[derive(PartialEq)]] this
// will require for all the struct fields to be equal as opposed to the custome logic we've defined below.
impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

fn main() {
    // The PartialEq trait establishes equality between two values.
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:30");
    let c = Flight::new("New York", "Riyadh", "08:00");
    println!("{}", a == b); // the PartialEq trait allows us to check for equality
    println!("{}", b.eq(&c)); // eq methods checks is equal
    println!("{}", b.ne(&a)); // ne methods checks is not equal.
}
