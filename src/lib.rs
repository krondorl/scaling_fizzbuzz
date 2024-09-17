// Copyright 2024 Adam Burucs. MIT license.

pub mod helpers {
    use config::*;
    use std::collections::HashMap;

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
                None => Err(String::from("Error: Cannot get max_iter value")),
            }
        } else {
            Err(String::from(
                "Error: Cannot get key because HashMap is empty",
            ))
        }
    }
}
