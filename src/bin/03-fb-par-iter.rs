// Copyright 2024 Adam Burucs. MIT license.

use helpers::{print_vector, read_config};
use rayon::prelude::*;
use scaling_fizzbuzz::*;
use std::error::Error;

fn fizz_buzz_par_iter(numbers: &Vec<u32>) -> Result<Vec<String>, String> {
    if numbers.len() < 3 {
        return Err(String::from("Error: input parameter should be at least 3."));
    }

    let results: Vec<String> = numbers
        .par_iter()
        .map(|&i| {
            if i % 15 == 0 {
                "FizzBuzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else {
                i.to_string()
            }
        })
        .collect();

    Ok(results)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Scaling FizzBuzz");
    println!("Multithreaded version with Rayon");
    println!();

    let parsed_settings = read_config("config.json")?;
    let max_iter: u32 = parsed_settings.max_iter;

    let v: Vec<u32> = (1..=max_iter).collect();

    let fb = fizz_buzz_par_iter(&v);
    match fb {
        Ok(fb_vec) => print_vector(&fb_vec),
        Err(e) => println!("{e}"),
    }

    Ok(())
}
