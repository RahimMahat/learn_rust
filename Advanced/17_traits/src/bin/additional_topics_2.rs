use std::fmt::{Debug, Display, Formatter, Result};

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

// we know the idiomatic way of Implementing a Debug trait using compiler directive #[derive(Debug)]
// but we can also implement custom Debug trait just like Display implementation.
impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "AppleType::RedDelicious"),
            AppleType::GreenApples => write!(formatter, "AppleType::GreenApples"),
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

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        // write!(
        //     formatter,
        //     "Apple ::: [ Kind: {:?}, Price: {} ]",
        //     self.kind, self.price
        // )

        // The other way of outputing the debug string using formatter method.
        formatter
            .debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Price", &self.price)
            .finish()
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 12.34,
    };

    println!("Display: {}", lunch_snack);
    println!("Debug: {:?}", lunch_snack);
}
