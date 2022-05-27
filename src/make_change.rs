// Make change

pub struct Coins {
    pub pennies: u32,
    pub nickels: u32,
    pub dimes: u32,
    pub quarters: u32,
}

impl Coins {
    pub fn make_int(&self, input: f32) -> u32 {
        const DOLLAR_MAKER: f32 = 100.;
        let coin_amount: f32 = input * DOLLAR_MAKER;
        coin_amount as u32
    }

    pub fn make_quarters(&self, change: u32) -> u32 {
        const one_quarter: u32 = 25;
        &change / &one_quarter
    }

    pub fn after_quarters(&self, change: u32, num_of_quarters: u32) -> u32 {
        const one_quarter: u32 = 25;
        &change - &num_of_quarters * &one_quarter
    }

    pub fn make_dimes(&self, rem_change: u32) -> u32 {
        const one_dime: u32 = 10;
        &rem_change / &one_dime
    }

    pub fn after_dimes(&self, rem_change: u32, num_of_dimes: u32) -> u32 {
        const one_dime: u32 = 10;
        &rem_change - &num_of_dimes * &one_dime
    }

    pub fn make_nickels(&self, rem_change: u32) -> u32 {
        const one_nickel: u32 = 5;
        &rem_change / &one_nickel
    }

    pub fn after_nickels(&self, rem_change: u32, num_of_nickels: u32) -> u32 {
        const one_nickel: u32 = 5;
        &rem_change - &num_of_nickels * &one_nickel
    }

    pub fn make_pennies(&self, rem_change: u32) -> u32 {
        const one_penny: u32 = 1;
        &rem_change / &one_penny
    }
}
