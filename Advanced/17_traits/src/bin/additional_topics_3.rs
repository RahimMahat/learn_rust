use std::clone::Clone;

// you can give a compiler directive on a struct to derive Clone trait just like the Debug trait.
// this wll only work if all the field types in the struct implement clone trait
// #[derive(Clone)]
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

fn main() {
    let morning_appointment = Appointment::new("Dr. House", "9:00AM", "10:00AM");
    let replacement_appointment = morning_appointment.clone();
    println!(
        "{} is seeing the patient from {} to {}",
        replacement_appointment.doctor,
        replacement_appointment.start_time,
        replacement_appointment.end_time
    );
}
