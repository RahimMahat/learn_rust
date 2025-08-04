// This file will be convering the additional topics within the traits concept.

// A supertrait is a trait from which another trait inherits functionality. The parent is called
// the supertrait and teh child is called the subtrait.
// traits with generics.
trait Investment<T> {
    // A getter method is a method that retrieves a piece of data. It "gets" a piece of state.
    fn amount(&self) -> T;

    // A setter method is a method that writes a piece of data. It "sets" a piece of state.
    fn double_amount(&mut self) {}
}

// here Taxable becomes the subtrait which has Investment as it's supertrait.
trait Taxable: Investment<f64> {
    // trait associated constant.
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount * 2.0;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    amount: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount * 2.0;
    }
}

impl Taxable for Bonus {
    // this will override the original value of const which was 0.25
    const TAX_RATE: f64 = 0.50;
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

fn main() {
    let income = Income { amount: 874382.34 };
    println!("Total tax owed for income: {}", income.tax_bill());

    let mut bonus = Bonus { amount: 247843.32 };
    println!("Total tax owed for bonus: {}", bonus.tax_bill());
    bonus.double_amount();
    println!("Double amount of bonus: {}", bonus.amount);

    let mut weekend = QualityTime { minutes: 420 };
    weekend.double_amount();
    println!("weekend quality time {}", weekend.amount());
}
