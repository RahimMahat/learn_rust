// the name traits is the crate name define in Cargo.toml file
use traits::{
    lodging::{Accommodation, AirBnB, Description, Hotel},
    utils::{book_for_one_night, choose_best_place_to_stay, mix_and_match},
};

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

    let _luxe_hotel = choose_best_place_to_stay();

    // A trait object is an instance of a type that implements a particular trait whose methods will be accessed at runtime using a feature called dynamic dispatch.
    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    println!("{}", stays[0].get_description());
}
