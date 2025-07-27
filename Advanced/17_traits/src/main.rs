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

fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    // entity can be any type as long as it implements Accommodation trait.
    entity.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("Orchids");
    println!("{}", hotel.summarize());
    hotel.book("Piers", 5);

    let mut airbnb = AirBnB::new("Mark");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);

    // Hotel & AirBnB both struct implement Accommodation trait hence we can call the
    // book_for_one_night function on both of them.
    book_for_one_night(&mut hotel, "Quraish");
    book_for_one_night(&mut airbnb, "Quraish");

    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);
}
