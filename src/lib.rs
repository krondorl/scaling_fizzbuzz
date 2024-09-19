// Copyright 2024 Adam Burucs. MIT license.

pub mod helpers {
    use config::*;
    use std::collections::HashMap;
    use std::io::{self, BufWriter, Write};

    pub fn get_settings(config: &str) -> Result<HashMap<String, u32>, String> {
        let settings = Config::builder().add_source(config::File::with_name(config));

        match settings.build() {
            Ok(ok_settings) => {
                // Config OK
                let try_deser = ok_settings.try_deserialize::<HashMap<String, u32>>();
                match try_deser {
                    Ok(parsed_settings) => {
                        Ok(parsed_settings) // Parsing OK
                    }
                    Err(_) => Err(String::from("Error: Cannot parse configuration")),
                }
            }
            Err(_) => Err(String::from("Error: Cannot build configuration")),
        }
    }

    pub fn get_key(parsed_settings: &HashMap<String, u32>, key: &str) -> Result<u32, String> {
        if !parsed_settings.is_empty() {
            match parsed_settings.get(key) {
                Some(&value) => Ok(value),
                None => Err(String::from("Error: Cannot get key value")),
            }
        } else {
            Err(String::from(
                "Error: Cannot get key because HashMap is empty",
            ))
        }
    }

    pub fn print_vector<T: std::fmt::Debug>(vector: &Vec<T>) {
        for row in vector {
            println!("{:?}", row);
        }
    }

    pub fn print_vec_stdout<T: std::fmt::Debug + std::fmt::Display>(vector: &Vec<T>) {
        let stdout = io::stdout();
        let handle = stdout.lock(); // Lock stdout to avoid data races
        let mut buf_writer = BufWriter::new(handle);

        // Write multiple lines using buffered I/O
        for row in vector {
            match writeln!(buf_writer, "{}", row) {
                Ok(_) => {} // Successfully wrote the line, nothing to do
                Err(e) => {
                    eprintln!("Failed to write to buffer: {}", e);
                }
            }
        }

        // Explicitly flush the buffer to make sure all output is written
        match buf_writer.flush() {
            Ok(_) => {} // Successfully flushed
            Err(e) => {
                eprintln!("Failed to flush the buffer: {}", e);
            }
        }
    }
}
