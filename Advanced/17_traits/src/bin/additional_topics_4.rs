#[derive(PartialEq, Eq)]
struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

struct Flight {
    origin: String,
    destination: String,
    time: String,
}

#[derive(PartialEq)]
enum Musician {
    SingerSongWriter(String),
    Band(u32),
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

impl BusTrip {
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

// defining equality for different types. we're checking Flight struct type with BusTrip struct type
impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.origin == other.origin && self.time == other.time
    }
}

// just like struct you can define a custom implementation of PartialEq to enum type.
// impl PartialEq for Musician {
//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             Musician::SingerSongWriter(name) => match other {
//                 Musician::SingerSongWriter(other_name) => name == other_name,
//                 Musician::Band(_) => false,
//             },
//             Musician::Band(numbers) => match other {
//                 Musician::Band(other_numbers) => numbers == other_numbers,
//                 Musician::SingerSongWriter(_) => false,
//             },
//         }
//     }
// }
//

fn main() {
    // The PartialEq trait establishes equality between two values.
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:30");
    let c = Flight::new("New York", "Riyadh", "08:00");
    println!("{}", a == b); // the PartialEq trait allows us to check for equality
    println!("{}", b.eq(&c)); // eq methods checks is equal
    println!("{}", b.ne(&a)); // ne methods checks is not equal.

    let bus_trip = BusTrip::new("Tokyo", "Beijing", "08:00");
    let flight = Flight::new("Tokyo", "Seoul", "08:00");
    println!("{}", bus_trip == flight);
    // println!("{}", flight == bus_trip); // this will not work since we've not implmented PartialEq
    // for Flight on BusTrip. we also not have the regular PartialEq trait on BusTrip so we can't do bus_trip == bus_trip either.

    let musician_1 = Musician::SingerSongWriter("Rasputin".to_string());
    let musician_2 = Musician::SingerSongWriter("Zaz".to_string());
    let band_1 = Musician::Band(4);
    let band_2 = Musician::Band(4);

    println!("{}", musician_1 == musician_2);
    println!("{}", band_1 == band_2);

    // Eq trait is a subtrait of PartialEq which can only be implmented if
    // i. b1 == b1; 11. b1 == b2 implies b2 == b1 iii. b1 == b2 and b2 == b3 implies b1 == b3.
    let b1 = BusTrip::new("New Delhi", "Mumbai", "08:00");
    let b2 = BusTrip::new("New Delhi", "Mumbai", "08:00");
    let b3 = BusTrip::new("New Delhi", "Mumbai", "08:00");
    println!("{}", b1.eq(&b1));
    println!("{}", b1.eq(&b2) && b2.eq(&b1));
    println!("{}", b1 == b2 && b2 == b3 && b1 == b3);
}
