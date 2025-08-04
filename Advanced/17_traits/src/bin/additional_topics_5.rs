struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

// The PartialOrd trait indicates that a type can be ordered/sorted.
// PartialOrd is a subtrait of PartialEq.
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // if self.salary == other.salary {
        //     Some(std::cmp::Ordering::Equal)
        // } else if self.salary < other.salary {
        //     Some(std::cmp::Ordering::Less)
        // } else if self.salary > other.salary {
        //     Some(std::cmp::Ordering::Greater)
        // } else {
        //     None
        // }

        // A u32 by default implements PartialOrd trait so the if-else logic can be replace by.
        self.salary.partial_cmp(&other.salary)
    }
}

fn main() {
    let long_commute = Job {
        salary: 100_000,
        commute_time: 2,
    };
    let short_commute = Job {
        salary: 75_000,
        commute_time: 1,
    };

    println!("{}", long_commute.gt(&short_commute));
    println!("{}", long_commute < short_commute);
    println!("{}", long_commute >= short_commute);
    println!("{}", long_commute.le(&short_commute));
}
