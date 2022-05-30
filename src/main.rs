mod make_change;

use make_change::Coins;
use std::io;

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
        Err(_) => 0.0,
    };

    let mut rem_change: u32 = coin_choice.make_int(user_input);
    let quarter_count: u32 = coin_choice.make_quarters(rem_change);
    rem_change = coin_choice.after_quarters(rem_change, quarter_count);
    let dime_cnt: u32 = coin_choice.make_dimes(rem_change);
    rem_change = coin_choice.after_dimes(rem_change, dime_cnt);
    let nickel_cnt = coin_choice.make_nickels(rem_change);
    rem_change = coin_choice.after_nickels(rem_change, nickel_cnt);
    let penny_cnt = coin_choice.make_pennies(rem_change);

    println!("\nQuarters: {}", quarter_count);
    println!("Dimes: {}", dime_cnt);
    println!("Nickels: {}", nickel_cnt);
    if penny_cnt != 1 {
        println!("Pennies: {}", penny_cnt);
    } else {
        println!("Penny: {}", penny_cnt);
    }
}
