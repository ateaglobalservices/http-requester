use std::str::FromStr;
use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub requests_to_send: u64,
    pub method: String,
    pub uri: String,
    pub headers: Map<String, Value>,
    pub body: Value,
}

impl FromStr for Config {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config: Config = serde_json::from_str(&s)?;

        Ok(config)
    }
} 