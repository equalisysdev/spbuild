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

    //FIXME: When running this code, I get the following error:
    /*
     *   called `Result::unwrap()` on an `Err` value: Error("invalid type: map, expected a sequence", line: 9, column: 22)
     *   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
    
    // Result<String> -> String -> &str -> Config
    let config: Config = serde_json::from_str(&unserialized_string.unwrap()).unwrap();

    Ok(config)
}
