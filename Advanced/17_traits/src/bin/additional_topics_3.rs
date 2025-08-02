use std::clone::Clone;

// you can give a compiler directive on a struct to derive Clone trait just like the Debug trait.
// this wll only work if all the field types in the struct implement clone trait
// #[derive(Clone)]
#[derive(Debug)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

// implementing custom Clone trait on a struct.
impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            // just so we don't take ownership of the struct fields we're invoking .clone() method
            // on each of the fields.
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}

// The Copy trait is the subtrait of Clone trait. In other words Clone trait is the supertrait of the Copy trait.
// so if a type chooses implement Copy trait it has to implement Clone trait. But if a type chooses
// to implement Clone trait it does not necessarily need to implement Copy trait.
// instead of doing an impl block below to implement Copy trait you can use compiler directive here
// as well like this:  #[derive(Clone, Copy)]
#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

// Copy trait does not mandate any methods so you don't have to define any methods within the {}
// The only condition being here is that the field type must implement Copy trait which in our case is u32 and implements Copy trait.
impl Copy for Duration {}

fn main() {
    let morning_appointment = Appointment::new("Dr. House", "9:00AM", "10:00AM");
    let replacement_appointment = morning_appointment.clone();
    println!(
        "{} is seeing the patient from {} to {}",
        replacement_appointment.doctor,
        replacement_appointment.start_time,
        replacement_appointment.end_time
    );
    println!("{morning_appointment:?}"); // since we had implemented the Clone trait on the struct there was no move.

    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;
    println!("{:?}", another_hour);
    println!("{:?}", one_hour); // since Duration struct is implementing Copy trait you can access
    // the one_hour struct instance even after it has been reassigned to a different var.
}
