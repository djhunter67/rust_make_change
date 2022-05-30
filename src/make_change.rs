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
        &change / &self.quarters
    }

    pub fn after_quarters(&self, change: u32, num_of_quarters: u32) -> u32 {
        &change - &num_of_quarters * &self.quarters
    }

    pub fn make_dimes(&self, rem_change: u32) -> u32 {
        &rem_change / &self.dimes
    }

    pub fn after_dimes(&self, rem_change: u32, num_of_dimes: u32) -> u32 {
        &rem_change - &num_of_dimes * &self.dimes
    }

    pub fn make_nickels(&self, rem_change: u32) -> u32 {
        &rem_change / &self.nickels
    }

    pub fn after_nickels(&self, rem_change: u32, num_of_nickels: u32) -> u32 {
        &rem_change - &num_of_nickels * &self.nickels
    }

    pub fn make_pennies(&self, rem_change: u32) -> u32 {
        &rem_change / &self.pennies
    }
}
