use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::process;

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

    // The zip method.
    let first_names = ["Casey", "Robert", "Cargo"];
    let last_names = ["Johnson", "Smith", "Rustman"];

    for (first_name, last_name) in first_names.iter().zip(last_names) {
        println!("{} {}", first_name, last_name);
    }

    // The fold method.
    let earnings = [4, 7, 9, 13];
    let sum = earnings.into_iter().fold(0, |total, current| {
        println!("total {total}, current {current}");
        total + current
    });
    println!("{}", sum);

    // The reduce method.
    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("{:?}", sum);

    // The sum, product, max, min and count methods.
    let total: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("sum {}, product {}", total, product);
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    println!("max {}, min {}", max.unwrap(), min.unwrap());
    let count = numbers.iter().count();
    println!("count {}", count);

    // The last, nth, nth_back, and position methods.
    let performers = ["Rustful Five", "Rust in piece", "Rust n Python"];
    let last = performers.into_iter().last().unwrap();
    println!("{}", last);
    let nth = performers.into_iter().nth(1).unwrap();
    println!("{}", nth);
    let nth_back = performers.into_iter().nth_back(2).unwrap();
    println!("{}", nth_back);
    let position = performers
        .into_iter()
        .position(|performer| performer.contains("Python"))
        .unwrap();
    println!("{}", position);

    // The take, rev, skip and step_by methods.
    let fifty_numbers = 1..=50;
    let take_40 = fifty_numbers.take(40);
    let skip_5 = take_40.skip(5);
    for num in skip_5.step_by(10) {
        print!("{}/", num)
    }

    // The sort and sort_by_key methods.
    let mut points = [3, 8, 1, 11, 5];
    points.sort();
    println!("\n{}", points.is_sorted());

    // The line method.
    let contents = fs::read_to_string("notes.txt").unwrap();
    for line in contents.lines() {
        println!("{}", line);
    }
}

fn command_line_args() {
    #[derive(Debug)]
    struct Settings {
        video_file: String,
        subtitles: bool,
        high_definition: bool,
    }

    fn collect_settings() -> Settings {
        // collecting the args but skipping the file name which would be the first argument to the program by default. and then only take 3 elements that we need.
        let mut args = env::args().skip(1).take(3);

        let video_file = args.next().unwrap_or_else(|| {
            eprintln!("No video file specified");
            process::exit(1)
        });

        let mut settings = args.map(|setting| setting.parse::<bool>().unwrap_or(false));
        let subtitles = settings.next().unwrap_or(false);
        let high_definition = settings.next().unwrap_or(false);

        Settings {
            video_file,
            subtitles,
            high_definition,
        }
    }
    let settings = collect_settings();
    println!("{:?}", settings);
}

fn reading_directory() -> io::Result<()> {
    /*
    The fs::read_dir() function returns a io::Result<ReadDir> enum.
    The ReadDir struct implements the Iterator trait.
    The iterator yields Result<DirEntry, Error> enums.
    The DirEntry struct supports a 'path' method.
    The fs::metadata function returns a Metadata struct.
    The Metadata struct includes an 'is_file' method.
    the fs::read_to_string function returns a io::Result<String>.
    */

    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        if metadata.is_file() {
            println!("{:?}\n---------------------------------------- ", path);
            let contents = fs::read_to_string(&path)?;
            println!("{}", contents);
        }
    }

    Ok(())
}

fn main() {
    iteration();

    println!(
        "{:?}",
        count_chars("Sally sells sea shells by the sea shores")
    );

    adapter_methods();
    command_line_args();
    reading_directory();
}
