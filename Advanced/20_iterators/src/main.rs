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

    // The filter & copied method.
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|number| *number % 2 == 0)
        .copied()
        .collect();
    println!("{:?}", evens);

    // The find and rfind method.
    let first_odd = numbers.iter().find(|number| *number % 2 != 0);
    let last_odd = numbers.iter().rfind(|number| *number % 2 != 0);
    println!("{:?}, {:?}", first_odd, last_odd);

    // The any and all method.
    let all_are_even = numbers.iter().all(|number| number % 2 == 0);
    println!("all_are_even {}", all_are_even);
    let any_are_even = numbers.iter().any(|number| number % 2 == 0);
    println!("any_are_even {}", any_are_even);

    // The cloned method.
    let tees = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Matcha"),
    ];
    let more_tees: Vec<String> = tees
        .iter()
        .filter(|tee| tee.contains("Hot"))
        .cloned()
        .collect();
    println!("{:?}", more_tees);

    // The filter_map method.
    let stocks = ["nvda", "", "aapl", "", "msft", "goog"];
    let capitalized_stock = stocks
        .iter()
        .filter_map(|stock| {
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect::<Vec<String>>();
    println!("{:?}", capitalized_stock);

    // The flatten method.
    let spreadsheet = vec![[100, 200, 300], [123, 223, 323], [987, 654, 321]];
    let values: Vec<i32> = spreadsheet.into_iter().flatten().collect();
    println!("{:?}", values);

    // The flat_map method.
    let attendees = ["Bob", "Mary, Kevin", "Mike, Robin, Bruce"];
    let attendees: Vec<&str> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();
    println!("{attendees:?}");

    // The enumerate method.
    let applicants = vec!["Rob", "Bob", "Alex", "Rich", "John", "Dan"];
    let winners = applicants
        .into_iter()
        .enumerate()
        // .filter(|(index, _)| index % 3 == 0)
        // .map(|(_, applicant)| applicant)
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();
    println!("{:?}", winners);

    // The partition method.
    // let groups: (Vec<i32>, Vec<i32>) = numbers.into_iter().partition(|number| number % 2 == 0);
    let (evens, odds): (Vec<i32>, Vec<i32>) =
        numbers.into_iter().partition(|number| number % 2 == 0);
    println!("evens {:?}, odds {:?}", evens, odds);
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
