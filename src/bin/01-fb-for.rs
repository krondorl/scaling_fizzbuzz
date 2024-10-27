// Copyright 2024 Adam Burucs. MIT license.

use helpers::read_config;
use scaling_fizzbuzz::*;
use std::error::Error;

fn fizz_buzz_for(n: u32) {
    if n < 3 {
        println!("Error: input parameter should be at least 3.");
        return;
    }
    for i in 1..=n {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Scaling FizzBuzz");
    println!("For loop version");
    println!();

    let parsed_settings = read_config("config.json")?;
    let max_iter: u32 = parsed_settings.max_iter;

    fizz_buzz_for(max_iter);

    Ok(())
}
