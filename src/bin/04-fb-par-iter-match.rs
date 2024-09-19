// Copyright 2024 Adam Burucs. MIT license.

use helpers::{get_key, get_settings, print_vector};
use rayon::prelude::*;
use scaling_fizzbuzz::*;
use std::collections::HashMap;

fn fizz_buzz_par_iter_match(numbers: &Vec<u32>) -> Result<Vec<String>, String> {
    if numbers.len() < 3 {
        return Err(String::from("Error: input parameter should be at least 3."));
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

    Ok(results)
}

fn main() {
    println!("Scaling FizzBuzz");
    println!("Multithreaded match version with Rayon");
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

    let fb = fizz_buzz_par_iter_match(&v);
    match fb {
        Ok(fb_vec) => print_vector(&fb_vec),
        Err(e) => println!("{e}"),
    }
}
