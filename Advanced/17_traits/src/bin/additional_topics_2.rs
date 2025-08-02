use std::fmt::{Display, Formatter, Result};

enum AppleType {
    RedDelicious,
    GreenApples,
}

// Implementing Display trait to our custom enum.
impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        // Representing each enum variant distinctly you can just provide one uniform output to
        // each variant, but just so we cover each differently we're doing this.
        match self {
            AppleType::RedDelicious => write!(formatter, "Red Apple"),
            AppleType::GreenApples => write!(formatter, "Green Apple"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

// Implementing Display trait to our custom struct
impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        // we're passing the desired string structure ("{} apple for {}") to the fmt::Formatter
        // struct in the write! macro which will return Result of our struct in that format.
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 12.34,
    };

    println!("{}", lunch_snack);
}
