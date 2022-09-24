mod config;

pub use config::Config;

use anyhow::{anyhow, Result};

pub fn run(config: Config) -> Result<()> {
    let contents = match std::fs::read_to_string(&config.filename) {
        Ok(v) => v,
        Err(_) => return Err(anyhow!("Could not read file: {}", config.filename)),
    };

    let result = search(&config.query, &contents, config.case_sensitive);
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, sensitive: bool) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|&line| match sensitive {
            true => line.contains(&query),
            false => line.to_lowercase().contains(&query),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}
