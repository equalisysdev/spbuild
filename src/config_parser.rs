use std::vec::Vec;

use std::path::Path;
use std::fs::read_to_string;

use serde::Deserialize;

use crate::project::Project;

#[derive(Deserialize)]
pub struct Config {
    pub projects: Vec<Project>,
}

pub fn parse_config(path: &Path) -> Result<Config, String> {

    let unserialized_string = read_to_string(path);

    // Error handling
    if (&unserialized_string).is_err() {
        return Err(format!("Failed to read config file: {}", path.display()));
    }
    
    // Result<String> -> String -> &str -> Config
    let contents = read_to_string(path)
        .map_err(|e| format!("Failed to read config file `{}': {}", path.display(), e))?;

    let config: Config = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse config file `{}': {}", path.display(), e))?;

    Ok(config)
}
