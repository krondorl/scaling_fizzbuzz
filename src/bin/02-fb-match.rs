// Copyright 2024 Adam Burucs. MIT license.

use helpers::{print_vector, read_config};
use scaling_fizzbuzz::*;
use std::error::Error;

fn fizz_buzz_match(n: u32) -> Result<Vec<String>, String> {
    if n < 3 {
        return Err(String::from("Error: input parameter should be at least 3."));
    }
    let mut result: Vec<String> = Vec::new();
    for i in 1..=n {
        match i % 15 {
            0 => result.push(String::from("FizzBuzz")),
            3 | 6 | 9 | 12 => result.push(String::from("Fizz")),
            5 | 10 => result.push(String::from("Buzz")),
            _ => {
                result.push(i.to_string());
            }
        };
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Scaling FizzBuzz");
    println!("Match loop version");
    println!();

    let parsed_settings = read_config("config.json")?;
    let max_iter: u32 = parsed_settings.max_iter;

    let fb = fizz_buzz_match(max_iter);
    match fb {
        Ok(fb_vec) => print_vector(&fb_vec),
        Err(e) => println!("{e}"),
    }

    Ok(())
}
