// Copyright 2024 Adam Burucs. MIT license.

pub mod helpers {
    use serde::Deserialize;
    use std::fs;
    use std::io::{self, BufWriter, Write};

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub max_iter: u32,
    }

    pub fn read_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(filename)?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
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
