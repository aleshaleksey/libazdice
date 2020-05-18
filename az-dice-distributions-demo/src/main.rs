// This is a mini-program which demonstrates the use of `libazdice`.
extern crate libazdice;
use std::io;
use libazdice::parse;

fn main() {
    let reps = if let Some(n) = std::env::args().nth(1) {
        n.parse::<usize>().expect(&format!("Argument must be numeric! This is not: {}.",n))
    } else {
        10_000
    };

    println!("Hello, world! Lets make a dice distribution. Input the dice string:");
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Could not read input: {}", e);
        return;
    }

    // Parse input into dicebag.
    let dice_bag = match parse::parse(input.clone()) {
        Ok(db) => db,
        Err(e) => {
            println!("Could not parse input to dice:{:?}",e);
            return;
        }
    };

    let probability_distribution = dice_bag.make_frequency_distribution(reps);

    println!("Result for {} rolls of ({})",reps, input.trim());
    println!("Value\t| Percentage");
    for (i,n) in probability_distribution { println!("{}\t|  {}",i,n); }
}
