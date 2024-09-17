// Copyright 2024 Adam Burucs. MIT license.

use helpers::{get_key, get_settings};
use scaling_fizzbuzz::*;
use std::collections::HashMap;

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

fn main() {
    println!("Scaling FizzBuzz");
    println!("For loop version");
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

    fizz_buzz_for(max_iter);
}
