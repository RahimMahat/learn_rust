fn main() {
    let cities = vec![
        String::from("Riyadh"),
        String::from("Jeddah"),
        String::from("Doha"),
    ];

    let favourite_cities = &cities[..2];
    let places = cities;
    // println!("{favourite_cities:?}"); // dangling reference.
}
