// This is a mini-program which demonstrates the use of `libazdice`.
extern crate libazdice;
use libazdice::parse;
use std::io;

fn main() {
    loop {
        println!("Hello, world! Lets make a dice. Input the dice string:");
        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Could not read input: {}", e);
            return;
        }
        input = input.trim().to_owned();

        if input.is_empty() {
            input = "1d20".to_owned();
        }

        // Parse input into dicebag.
        let dice_bag = match parse::parse(input.clone()) {
            Ok(db) => db,
            Err(e) => {
                println!("Could not parse input to dice:{:?}", e);
                continue;
            }
        };
        println!("Dicebag: {}", dice_bag);
        let roll = dice_bag.roll();
        println!(
            "You rolled {}, and got {}.\n",
            input,
            roll.total()
        );
        println!("Result: {}", roll);
    }
}
