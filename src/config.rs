use serde::{Deserialize, Serialize};

//configuration for the database
//just holds where to save the data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub storage_path: String,
}

impl Config {
    //create a new config with a custom path
    pub fn new(path: String) -> Self {
        Self { storage_path: path }
    }
}

impl Default for Config {
    // default saves to ./data directory
    fn default() -> Self {
        Self::new("./data".to_string())
    }
}
