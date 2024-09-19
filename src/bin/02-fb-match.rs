// Copyright 2024 Adam Burucs. MIT license.

use helpers::{get_key, get_settings, print_vector};
use scaling_fizzbuzz::*;
use std::collections::HashMap;

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

fn main() {
    println!("Scaling FizzBuzz");
    println!("Match loop version");
    println!();

    let settings = get_settings("config");
    let mut parsed_settings: HashMap<String, u32> = HashMap::new();
    let mut max_iter: u32 = 1000;

    match settings {
        Ok(parsed_deser) => parsed_settings = parsed_deser,
        Err(e) => println!("{e}"),
    }

    println!("{:#?}", parsed_settings);

    let getting_key = get_key(&parsed_settings, "max_iter");

    match getting_key {
        Ok(value) => max_iter = value,
        Err(e) => println!("{e}"),
    }

    let fb = fizz_buzz_match(max_iter);
    match fb {
        Ok(fb_vec) => print_vector(&fb_vec),
        Err(e) => println!("{e}"),
    }
}
