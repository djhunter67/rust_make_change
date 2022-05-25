// Make change

pub struct Coins {
    pub pennies: u32,
    pub nickels: u32,
    pub dimes: u32,
    pub quarters: u32,
}

impl Coins {
    pub fn make_coins(&self, change: f32) -> u32 {
        const DOLLAR_MAKER: u32 = 100;
        change as u32 * DOLLAR_MAKER 
    }
}
