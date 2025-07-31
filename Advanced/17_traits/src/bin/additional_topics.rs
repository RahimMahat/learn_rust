// This file will be convering the additional topics within the traits concept.

trait Taxable {
    // trait associated constant.
    const TAX_RATE: f64 = 0.25;

    // A getter method is a method that retrieves a piece of data. It "gets" a piece of state.
    fn amount(&self) -> f64;

    // A setter method is a method that writes a piece of data. It "sets" a piece of state.
    fn set_amount(&mut self, new_amount: f64);
    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

#[derive(Debug)]
struct Bonus {
    amount: f64,
}

impl Taxable for Bonus {
    // this will override the original value of const which was 0.25
    const TAX_RATE: f64 = 0.50;
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

fn main() {
    let income = Income { amount: 874382.34 };
    println!("Total tax owed for income: {}", income.tax_bill());

    let mut bonus = Bonus { amount: 247843.32 };
    println!("Total tax owed for bonus: {}", bonus.tax_bill());
    bonus.double_amount();
    println!("Double amount of bonus: {}", bonus.amount);
}
