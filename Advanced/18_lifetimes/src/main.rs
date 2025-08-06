// A function can not return a reference to the any owned value or parameter.
// fn create(number: i32) -> &i32 {
//     &number
// }

// the only way you can return a reference from a function is if it's coming from the parameter type.
// i.e. if the parameter type & return type both are same.
fn create(valid: &i32) -> &i32 {
    &valid
}

// generic lifetime annotation
// 'a is the lifetime annotation syntax. which in below example bounds the return reference
// lifetime with the lifetime of it's referent items's lifetime i.e the returned slice can not
// outlive the items collection
fn generic_lifetime<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("Riyadh"),
        String::from("Jeddah"),
        String::from("Doha"),
    ];

    let _favourite_cities = &cities[..2];
    let _places = cities;
    // println!("{favourite_cities:?}"); // dangling reference.
    // create(10);
    create(&3);

    let items = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];
    generic_lifetime(&items);
}
