mod make_change;

use std::io;
use make_change::Coins;
use math::round;

fn main() {
    let coin_choice: Coins = Coins {
        pennies: 1,
        nickels: 5,
        dimes: 10,
        quarters: 25,
    };

    println!("Please enter a dollar and change amount: ");
    let mut user_input: String = String::new();

    io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read line");

    let user_input: f32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => println!("Error, incorrect input"),
        };
        
    let correct_change: u32 = coin_choice.make_coins(user_input);

    

}
