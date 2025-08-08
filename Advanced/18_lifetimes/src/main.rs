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

// Multiple reference parameter
// the function will not compile unless we have explicitly defined the lifetime annotations.
fn choose_favourite<'a>(first: &'a str, second: &str) -> &'a str {
    println!("{second}");
    first
}
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

struct DenitstAppointment {
    doctor: String,
}

impl DenitstAppointment {
    fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
        // The function definition is similar to saying
        // fn book<'a, 'b, 'c>(&'a self, 'b check_in_time: &str, 'c check_out_time: str) -> &'a str
        // but due to the Third Elision rule we don't have to write out the lifetime annotations
        println!(
            "You are booked from {} to {} with doctor {}",
            check_in_time, check_out_time, self.doctor
        );
        &self.doctor
    }

    fn check_out_time<'a>(&self, check_in_time: &str, check_out_time: &'a str) -> &'a str {
        // but if you want to return a perticular reference that is not the instance then you have
        // to specify the lifetime annotation for the function parameter and the return parameter as well.
        check_out_time
    }
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

    let first = "Python";
    let second = "Rust";
    choose_favourite(first, second);
    println!("{}", longest(first, second));

    // third elision rule example
    let appt = DenitstAppointment {
        doctor: String::from("David"),
    };
    let result = appt.book("2:00", "3:00");
    // drop(appt);
    // println!("{result}"); // now this will be a dangling reference since the return lifetime of the
    // method is tied to the referent and can not outlive the referent.
    let check_out_time = appt.check_out_time("2:00", "3:00");
    drop(appt);
    println!("{}", check_out_time); // this will work fine.
}
