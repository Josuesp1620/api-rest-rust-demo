use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// A structure that manages environment variables
pub struct EnvManager;

impl EnvManager {
    /// Converts a HashMap of environment variables into a valid string for a .env file
    ///
    /// # Parameters:
    /// - env_map: HashMap<String, String> - Map of environment variables
    ///
    /// # Returns:
    /// - A string representing the environment variables
    pub fn env_map_to_string(env_map: &HashMap<String, String>) -> String {
        env_map.iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Converts a string of environment variables into a HashMap
    ///
    /// # Parameters:
    /// - env_string: String - String of environment variables
    ///
    /// # Returns:
    /// - HashMap<String, String> - Map of environment variables
    pub fn env_string_to_map(env_string: &str) -> HashMap<String, String> {
        let mut env_map = HashMap::new();
        
        for line in env_string.lines() {
            if line.starts_with('#') {
                continue; // Ignore comments
            }

            let mut parts = line.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                env_map.insert(key.to_string(), value.to_string());
            }
        }

        env_map
    }

    /// Creates a .env file from a HashMap
    ///
    /// # Parameters:
    /// - path: &str - The path where the file will be created
    /// - filename: &str - The name of the file
    /// - content: HashMap<String, String> - The environment variables to be written to the file
    ///
    /// # Returns:
    /// - Result<(), io::Error> - The result of the operation (Ok if successful, Error if there was an issue)
    pub fn create_env_file(path: &str, filename: &str, content: HashMap<String, String>) -> io::Result<()> {
        let full_path = Path::new(path).join(filename);

        // Convert the HashMap to a string
        let env_string = Self::env_map_to_string(&content);

        // Create the file and write the content
        let mut file = File::create(full_path)?;
        file.write_all(env_string.as_bytes())?;
        
        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Example usage
    let mut env_map = HashMap::new();
    env_map.insert("KEY1".to_string(), "value1".to_string());
    env_map.insert("KEY2".to_string(), "value2".to_string());

    // Convert the HashMap to a string
    let env_string = EnvManager::env_map_to_string(&env_map);
    println!("Env map to string:\n{}", env_string);

    // Convert the string back to a HashMap
    let parsed_map = EnvManager::env_string_to_map(&env_string);
    println!("Env string to map:");
    for (key, value) in parsed_map {
        println!("{} = {}", key, value);
    }

    // Create a .env file with the HashMap content
    EnvManager::create_env_file("./", ".env", env_map)?;

    Ok(())
}
