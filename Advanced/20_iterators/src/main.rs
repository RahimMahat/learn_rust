use std::collections::HashMap;

fn iteration() {
    // Full ownership
    // for value in collection
    // for value in collection.into_iter()
    let bools = vec![true, true, false, true, false, true];
    for boolean in bools {
        println!("{boolean}");
    }

    // Immutable reference
    // for value in &collection
    // for value in collection.iter()
    let numbers = vec![4, 8, 15, 16, 23, 42];
    for number in &numbers {
        println!("{number}");
    }
    println!("{numbers:?}");

    // Mutable reference.
    // for value in &mut collection
    // for value in collection.iter_mut()
    let mut flavours = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Caramel"),
    ];
    for flavour in &mut flavours {
        flavour.push_str(" Ice Cream");
    }

    println!("{flavours:?}");
}

fn count_chars(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    // for word in words {
    //     for character in word.chars() {
    //         let count = counts.entry(character).or_insert(0);
    //         *count += 1;
    //     }
    // }
    //The for_each() method allows you to iterate over each of the iterable element and perform
    //actions on it accpeting a closure as an orgument for element manupilation.
    //so replacing the above logic would look like this.
    words.for_each(|word| {
        word.chars().for_each(|character| {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        });
    });

    counts
}

fn adapter_methods() {
    // The map method.
    let numbers = [4, 8, 15, 16, 23, 42];

    let squares = numbers.into_iter().map(|number: i32| number.pow(2));
    for square in squares {
        print!("{} ", square)
    }

    // The collect method.
    let cubes: Vec<_> = numbers.iter().map(|number: &i32| number.pow(3)).collect();
    println!("\n{:?}", cubes);
}

fn main() {
    // iteration();
    //
    // println!(
    //     "{:?}",
    //     count_chars("Sally sells sea shells by the sea shores")
    // );
    //
    adapter_methods();
}
