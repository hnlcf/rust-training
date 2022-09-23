use std::env;

use anyhow::{anyhow, Result};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn try_parse_config(mut args: env::Args) -> Result<Config> {
        args.next();
        let query = args.next().ok_or(anyhow!("Didn't get a query string"))?;
        let filename = args.next().ok_or(anyhow!("Didn't get a filename"))?;
        let case_sensitive = match args.next() {
            Some(str) => {
                if str == "true" {
                    true
                } else {
                    false
                }
            }
            None => env::var("CASE_SENSITIVE").is_err(),
        };
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
