use std::error::Error;
use std::fs;
use std::io::{self, BufRead};

pub use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String;

    match config.content {
        None => content = fs::read_to_string(config.path)?,
        Some(v) => content = v,
    }

    let result: Vec<&str>;

    if config.ignore_case {
        result = search_case_insensitive(&config.query, &content)
    } else {
        result = search(&config.query, &content);
    }

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    content: &'a str,
) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

pub fn get_piped() -> String {
    let mut input = String::new();

    loop {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        if atty::is(atty::Stream::Stdin) {
            return input;
        }

        if let Ok(n) = handle.read_line(&mut input) {
            if n == 0 {
                return input;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = String::from("duct");
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
