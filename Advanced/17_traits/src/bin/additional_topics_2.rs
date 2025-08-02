use std::fmt::{Display, Formatter, Result};

struct Apple {
    kind: String,
    price: f64,
}

// Implementing Display trait to our custom struct
impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        // we're passing the desired string structure ("{} apple for {}") to the fmt::Formatter
        // struct in the write! macro which will return Result of our struct in that format.
        write!(formatter, "{} apple for {}", self.kind, self.price)
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: String::from("Regular"),
        price: 12.34,
    };

    println!("{}", lunch_snack);
}

