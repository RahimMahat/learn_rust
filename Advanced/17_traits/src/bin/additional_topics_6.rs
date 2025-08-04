use std::{ops::Add, process::Output};

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

impl Add for Lunch {
    // An associated type is a placeholder for a type that is required within a trait.
    type Output = f64;

    fn add(self, rhs: Self) -> Self::Output {
        self.cost + rhs.cost
    }
}

// we're saying here that we're defining a Generic type T that can be any type that implements Add
// trait and in addition the associated type of Add is also set to T so that Rust knows that the
// Output is going to be of same type as T.
fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let lunch_1 = Lunch { cost: 23.99 };
    let lunch_2 = Lunch { cost: 34.50 };
    println!("{:.2}", lunch_1 + lunch_2);

    println!("{}", add_two_numbers(3, 4));
    println!("{:.2}", add_two_numbers(43.14, 43.123));
}
