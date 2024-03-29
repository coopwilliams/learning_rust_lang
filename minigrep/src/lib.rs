//! # Minigrep
//! 
//! `minigrep` is a toy implementation of `grep` in Rust.

use std::{fs, env, error::Error};

pub struct Config {
    query: String,
    file_path : String,
    ignore_case: bool,
}

impl Config {
    pub fn build(
            mut args: impl Iterator<Item = String>, 
        ) -> Result<Config, &'static str> {
            
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let file_path = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path"),
            };

            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {query, file_path, ignore_case})
        }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read the file
    let contents = fs::read_to_string(config.file_path)?;
        // .expect("Should have been able to read the file");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Searches a string of contents for a substring.
/// 
/// # Example
/// 
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// 
/// assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        // huh. This compiles even if 
        // search() doesn't exist yet.
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        // huh. This compiles even if 
        // search() doesn't exist yet.
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}