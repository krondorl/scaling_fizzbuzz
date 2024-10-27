// Copyright 2024 Adam Burucs. MIT license.

use helpers::{print_vector, read_config};
use scaling_fizzbuzz::*;
use std::{borrow::Cow, error::Error};

fn fizz_buzz_match(n: u32) -> Result<Vec<Cow<'static, str>>, String> {
    if n < 3 {
        return Err(String::from("Error: input parameter should be at least 3."));
    }
    let n_as_usize = n.try_into();
    match n_as_usize {
        Ok(n) => {
            let mut result: Vec<Cow<'static, str>> = Vec::with_capacity(n);
            for i in 1..=n {
                match i % 15 {
                    0 => result.push(Cow::Borrowed("FizzBuzz")),
                    3 | 6 | 9 | 12 => result.push(Cow::Borrowed("Fizz")),
                    5 | 10 => result.push(Cow::Borrowed("Buzz")),
                    _ => {
                        result.push(Cow::Owned(i.to_string()));
                    }
                };
            }
            Ok(result)
        }
        Err(_) => Err(String::from("Error: can't convert n into usize.")),
    }
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
