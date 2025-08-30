use rand::{Rng, random, rng, seq::SliceRandom};

fn main() {
    let random_float: f32 = random();
    println!("random float {}", random_float * 100.0);

    let random_int = random::<u8>();
    println!("random int {}", random_int);

    let mut my_rng = rng();
    let random_f64 = my_rng.random::<f64>();
    println!("random f64 {}", random_f64 * 100.0);

    let ten_random_values = (0..10).map(|_| my_rng.random::<i8>()).collect::<Vec<i8>>();
    println!("ten_random_values {:?}", ten_random_values);

    let random_number = my_rng.random_range(29..53);
    println!("random_number within range 29 to 53 is {}", random_number);

    let random_bool = my_rng.random_bool(0.4);
    println!(
        "random_bool with 40% chace of getting true is {}",
        random_bool
    );

    let mut candies = vec![
        "Sour Patch kids",
        "Kit Kat",
        "Dair Milk",
        "Twix",
        "5 Star",
        "Burst",
    ];

    candies.shuffle(&mut my_rng);
    println!(
        "candies vector in random order (after shuffling) {:?}",
        candies
    );
}
