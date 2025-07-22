use std::collections::{HashMap, HashSet};

fn main() {
    // HashMap
    let mut menu: HashMap<String, f32> = HashMap::new();
    menu.insert("Stake".to_string(), 29.99);
    menu.insert("Tuna".to_string(), 19.99);
    menu.insert("Tikka Masala".to_string(), 29.99);
    println!("{menu:?}");

    let mut country_capitals = HashMap::<&str, &str>::new();
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");
    println!("{:?}", country_capitals);

    let data = [("Bobby", 7), ("Grant", 4), ("Ben", 5)];
    let mut employee_years = HashMap::from(data);
    println!("{employee_years:?}");

    employee_years.remove("Ben");
    println!("{employee_years:?}");

    let menu_item_price = menu["Stake"];
    println!("menu_item_price {menu_item_price}");
    let capital = country_capitals.get("France");
    match capital {
        Some(v) => println!("Capital of France is {}", v),
        None => println!("Key does not exist"),
    };

    employee_years.insert("Ben", 3); // this replaces the existing key-value
    println!("{employee_years:?}");

    employee_years.entry("Grant").or_insert(5); // .entry checks if a key exists and
    // then it has a method called .or_insert which will insert the value if the key does not exist
    employee_years.entry("Rich").or_insert(8); // the above line will not modify
    // anything since the key exists but this line will insert a new key-value pair.
    println!("{employee_years:?}");

    // HasSet.
    let mut concert_queue: HashSet<&str> = HashSet::new();
    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("length of hashset {}", concert_queue.len());
    concert_queue.insert("Molly"); // HashSet won't allow duplicates it won't throw error but also
    // won't store any duplicate entries in the collection.
    println!("{concert_queue:?}");

    // .remove will remove the existing key and returns true but if the key doesn't exist then it returns false.
    // .contains checks if a key exists and returns boolean.
    // .get returns Options enum if key exists then Some variant if it doesn't then None variant.

    let mut movie_queue: HashSet<&str> = HashSet::new();
    movie_queue.insert("Megan");
    movie_queue.insert("Phil");

    println!("union result {:?}", concert_queue.union(&movie_queue));
    println!(
        "difference result {:?}",
        concert_queue.difference(&movie_queue)
    );
    println!(
        "symmetric_difference result {:?}",
        concert_queue.symmetric_difference(&movie_queue)
    );
    println!(
        "is_disjoint result {:?}",
        concert_queue.is_disjoint(&movie_queue)
    );
    println!(
        "is_subset result {:?}",
        concert_queue.is_subset(&movie_queue)
    );
}
