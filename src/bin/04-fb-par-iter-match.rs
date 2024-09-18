// Copyright 2024 Adam Burucs. MIT license.

use helpers::{get_key, get_settings};
use rayon::prelude::*;
use scaling_fizzbuzz::*;
use std::collections::HashMap;

fn fizz_buzz_par_iter(numbers: &Vec<u32>) {
    if numbers.len() < 3 {
        println!("Error: input parameter should be at least 3.");
        return;
    }

    let results: Vec<String> = numbers
        .par_iter()
        .map(|&i| match i % 15 {
            0 => "FizzBuzz".to_string(),
            3 | 6 | 9 | 12 => "Fizz".to_string(),
            5 | 10 => "Buzz".to_string(),
            _ => i.to_string(),
        })
        .collect();

    for result in results {
        println!("{}", result);
    }
}

fn main() {
    println!("Scaling FizzBuzz");
    println!("Multithreaded version with Rayon");
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

    let v: Vec<u32> = (1..=max_iter).collect();

    fizz_buzz_par_iter(&v);
}
