use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay!")
    }

    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: Vec::new(),
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    // entity can be any type as long as it implements Accommodation trait. trait for function parameter syntax eg. fn name(var: impl Accommodation)
    // trait bound syntax is something that requires a generic type to implement a specific trait. eg. fn name<T: Accommodation>(var: T)
    entity.book(guest, 1);
}

fn mix_and_match(first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

fn main() {
    let mut hotel = Hotel::new("Orchids");
    println!("{}", hotel.summarize());
    hotel.book("Quraish", 5);

    let mut airbnb = AirBnB::new("Mark");
    println!("{}", airbnb.get_description());
    airbnb.book("Quraish", 3);

    // Hotel & AirBnB both struct implement Accommodation trait hence we can call the
    // book_for_one_night function on both of them.
    book_for_one_night(&mut hotel, "Piers");
    book_for_one_night(&mut airbnb, "Piers");

    mix_and_match(&mut hotel, &mut airbnb, "Sheikh");
    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);
}
